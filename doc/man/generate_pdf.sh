#!/bin/bash
#
# PDF Documentation Generator for UAV Swarm
#
# Converts Markdown documentation to PDF format
# Supports Mermaid diagrams conversion
#
# Usage:
#   ./generate_pdf.sh                    # Generate all PDFs
#   ./generate_pdf.sh USER_GUIDE.md      # Generate specific PDF
#   ./generate_pdf.sh --clean            # Clean generated files
#
# Requirements:
#   - pandoc (>= 2.0)
#   - mermaid-cli (mmdc)
#   - texlive-latex-base (for PDF generation)
#

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="$SCRIPT_DIR/pdf"
TEMP_DIR="$SCRIPT_DIR/.pdf_tmp"
LOG_FILE="$OUTPUT_DIR/generation.log"

# mermaid-cli & puppeteer configuration
PUPPETEER_EXECUTABLE_PATH="/usr/bin/google-chrome-stable"
PUPPETEER_ARGS="--no-sandbox --disable-setuid-sandbox"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Markdown files to convert
MARKDOWN_FILES=(
    "USER_GUIDE.md"
    "DEPLOYMENT_GUIDE.md"
    "CHANGELOG.md"
    "README.md"
)

#######################################
# Logging functions
#######################################

log() {
    echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$LOG_FILE"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_FILE"
}

#######################################
# Check dependencies
#######################################

check_dependencies() {
    log "Checking dependencies..."

    local missing_deps=0

    # Check pandoc
    if ! command -v pandoc &> /dev/null; then
        log_error "pandoc is not installed"
        echo "Install with: brew install pandoc (macOS) or apt install pandoc (Ubuntu)"
        missing_deps=1
    else
        log_success "pandoc found: $(pandoc --version | head -n1)"
    fi

    # Check mermaid-cli
    if ! command -v mmdc &> /dev/null; then
        log_warning "mermaid-cli (mmdc) is not installed - Mermaid diagrams will be skipped"
        echo "Install with: npm install -g @mermaid-js/mermaid-cli"
        MERMAID_AVAILABLE=false
    else
        log_success "mermaid-cli found: $(mmdc --version)"
        MERMAID_AVAILABLE=true
    fi

    # Check LaTeX (optional but recommended for better PDFs)
    if ! command -v lualatex &> /dev/null; then
        log_warning "LuaLaTeX not found - using simplified PDF generation"
        echo "For better PDFs, install: brew install basictex (macOS) or apt install texlive-latex-base (Ubuntu)"
        USE_LATEX=false
    else
        log_success "LaTeX found"
        USE_LATEX=true
    fi

    if [ $missing_deps -eq 1 ]; then
        log_error "Missing required dependencies. Exiting."
        exit 1
    fi

    log_success "Dependency check complete"
}

#######################################
# Setup directories
#######################################

setup_directories() {
    
    # log function output dir creation
    mkdir -p "$OUTPUT_DIR"

    log "Setting up directories..."

    mkdir -p "$TEMP_DIR"

    # Initialize log file
    echo "=== PDF Generation Log - $(date) ===" > "$LOG_FILE"

    log_success "Directories ready"
}

#######################################
# Extract and convert Mermaid diagrams
#######################################

extract_mermaid_diagrams() {
    local input_file="$1"
    local temp_file="$2"

    if [ "$MERMAID_AVAILABLE" = false ]; then
        log_warning "Skipping Mermaid conversion (mmdc not available)"
        cp "$input_file" "$temp_file"
        return
    fi

    log "Extracting Mermaid diagrams from $(basename "$input_file")..."

    local diagram_count=0
    local in_mermaid=false
    local mermaid_content=""
    local output_content=""

    while IFS= read -r line; do
        if [[ "$line" == '```mermaid' ]]; then
            in_mermaid=true
            mermaid_content=""
        elif [[ "$line" == '```' ]] && [ "$in_mermaid" = true ]; then
            in_mermaid=false
            diagram_count=$((diagram_count + 1))

            # Save mermaid content to temp file
            local mermaid_file="$TEMP_DIR/diagram_${diagram_count}.mmd"
            local png_file="$TEMP_DIR/diagram_${diagram_count}.png"

            echo "$mermaid_content" > "$mermaid_file"

            # Convert to PNG
            if mmdc -i "$mermaid_file" -o "$png_file" -b transparent 2>> "$LOG_FILE"; then
                log_success "Converted Mermaid diagram $diagram_count"
                # Replace mermaid block with image
                output_content+="![Diagram $diagram_count]($png_file)"$'\n'
            else
                log_error "Failed to convert Mermaid diagram $diagram_count"
                # Keep original mermaid block as code
                output_content+='```mermaid'$'\n'
                output_content+="$mermaid_content"
                output_content+='```'$'\n'
            fi
        elif [ "$in_mermaid" = true ]; then
            mermaid_content+="$line"$'\n'
        else
            output_content+="$line"$'\n'
        fi
    done < "$input_file"

    # Write processed content
    echo "$output_content" > "$temp_file"

    if [ $diagram_count -gt 0 ]; then
        log_success "Processed $diagram_count Mermaid diagram(s)"
    fi
}

#######################################
# Convert Markdown to PDF
#######################################

convert_to_pdf() {
    local input_file="$1"
    local output_file="$2"

    log "Converting $(basename "$input_file") to PDF..."

    # Create temporary processed file
    local temp_file="$TEMP_DIR/$(basename "$input_file")"

    # Extract and convert Mermaid diagrams
    extract_mermaid_diagrams "$input_file" "$temp_file"

    # Pandoc options
    local pandoc_opts=(
        --from markdown
        --to pdf
        --output "$output_file"
        --pdf-engine=lualatex
        --variable geometry:margin=1in
        --variable fontsize=11pt
        --variable documentclass=article
        --variable colorlinks=true
        --variable linkcolor=blue
        --variable urlcolor=blue
        --variable toccolor=gray
        --toc
        --toc-depth=3
        --number-sections
        --highlight-style=tango
        --metadata title="UAV Swarm Documentation"
        --metadata author="UAV Swarm Project"
        --metadata date="$(date +%Y-%m-%d)"
    )

    # Use simplified engine if LaTeX not available
    if [ "$USE_LATEX" = false ]; then
        pandoc_opts[3]="--pdf-engine=wkhtmltopdf"
    fi

    # Convert to PDF
    if pandoc "${pandoc_opts[@]}" "$temp_file" 2>> "$LOG_FILE"; then
        log_success "Generated PDF: $(basename "$output_file")"

        # Get file size
        local size=$(du -h "$output_file" | cut -f1)
        log "File size: $size"
    else
        log_error "Failed to generate PDF for $(basename "$input_file")"
        return 1
    fi
}

#######################################
# Generate all PDFs
#######################################

generate_all_pdfs() {
    log "Starting PDF generation for all documentation files..."

    local success_count=0
    local fail_count=0

    for md_file in "${MARKDOWN_FILES[@]}"; do
        local input_path="$SCRIPT_DIR/$md_file"
        local output_path="$OUTPUT_DIR/${md_file%.md}.pdf"

        if [ ! -f "$input_path" ]; then
            log_warning "File not found: $md_file (skipping)"
            continue
        fi

        echo ""
        if convert_to_pdf "$input_path" "$output_path"; then
            success_count=$((success_count + 1))
        else
            fail_count=$((fail_count + 1))
        fi
    done

    echo ""
    echo "======================================"
    log_success "PDF generation complete!"
    log "Successful: $success_count"
    [ $fail_count -gt 0 ] && log_error "Failed: $fail_count"
    log "Output directory: $OUTPUT_DIR"
    echo "======================================"
}

#######################################
# Generate single PDF
#######################################

generate_single_pdf() {
    local md_file="$1"
    local input_path="$SCRIPT_DIR/$md_file"
    local output_path="$OUTPUT_DIR/${md_file%.md}.pdf"

    if [ ! -f "$input_path" ]; then
        log_error "File not found: $md_file"
        exit 1
    fi

    log "Generating PDF for $md_file..."

    if convert_to_pdf "$input_path" "$output_path"; then
        log_success "PDF generated: $output_path"
    else
        log_error "Failed to generate PDF"
        exit 1
    fi
}

#######################################
# Clean generated files
#######################################

clean() {
    log "Cleaning generated files..."

    rm -rf "$OUTPUT_DIR"
    rm -rf "$TEMP_DIR"

    log_success "Cleanup complete"
}

#######################################
# Main script
#######################################

main() {
    echo ""
    echo "======================================"
    echo "  UAV Swarm - PDF Generator"
    echo "======================================"
    echo ""

    # Parse arguments
    if [ "$1" = "--clean" ]; then
        clean
        exit 0
    fi

    # Setup
    setup_directories
    check_dependencies

    echo ""

    # Generate PDFs
    if [ -z "$1" ]; then
        # Generate all PDFs
        generate_all_pdfs
    else
        # Generate specific PDF
        generate_single_pdf "$1"
    fi

    # Cleanup temp directory
    rm -rf "$TEMP_DIR"

    echo ""
    log "Log file: $LOG_FILE"
}

# Run main function
main "$@"

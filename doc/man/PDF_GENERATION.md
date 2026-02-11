# PDF Documentation Generation

Guide for generating PDF versions of the UAV Swarm documentation.

---

## Table of Contents

1. [Overview](#overview)
2. [Manual Generation](#manual-generation)
3. [Jenkins Automation](#jenkins-automation)
4. [Dependencies](#dependencies)
5. [Troubleshooting](#troubleshooting)

---

## Overview

The PDF generation system converts Markdown documentation to professional PDF files with:

- **Mermaid diagram conversion** - Automatically converts diagrams to images
- **Table of contents** - Auto-generated with hyperlinks
- **Syntax highlighting** - Code blocks with proper formatting
- **Cross-references** - Working internal links
- **Professional formatting** - Proper margins, fonts, and styling

### Generated PDFs

The script generates PDFs for:

- `USER_GUIDE.pdf` - Complete user guide with architecture diagrams
- `DEPLOYMENT_GUIDE.pdf` - Technical deployment guide
- `CHANGELOG.pdf` - Version history and migration guides
- `README.pdf` - Documentation index

PDFs are generated in: `doc/man/pdf/`

---

## Manual Generation

### Prerequisites

#### Required

1. **Pandoc** (>= 2.0)

```bash
# macOS
brew install pandoc

# Ubuntu/Debian
sudo apt-get install pandoc

# Verify installation
pandoc --version
```

2. **LaTeX Distribution** (for better PDF quality)

```bash
# macOS
brew install basictex

# Ubuntu/Debian
sudo apt-get install texlive-latex-base texlive-fonts-recommended

# Verify installation
pdflatex --version
```

#### Optional (for Mermaid diagrams)

3. **Mermaid CLI**

```bash
# Via npm (requires Node.js)
npm install -g @mermaid-js/mermaid-cli

# Verify installation
mmdc --version
```

**Note**: Without Mermaid CLI, diagrams will be rendered as code blocks in the PDF.

### Running the Script

#### Generate All PDFs

```bash
cd doc/man
./generate_pdf.sh
```

Output:
```
======================================
  UAV Swarm - PDF Generator
======================================

[INFO] Checking dependencies...
[SUCCESS] pandoc found: pandoc 2.19.2
[SUCCESS] mermaid-cli found: 10.6.1
[SUCCESS] LaTeX found
[SUCCESS] Dependency check complete

[INFO] Starting PDF generation for all documentation files...

[INFO] Converting USER_GUIDE.md to PDF...
[SUCCESS] Processed 8 Mermaid diagram(s)
[SUCCESS] Generated PDF: USER_GUIDE.pdf
[INFO] File size: 2.3M

...

======================================
[SUCCESS] PDF generation complete!
[INFO] Successful: 4
[INFO] Output directory: doc/man/pdf
======================================
```

#### Generate Specific PDF

```bash
./generate_pdf.sh USER_GUIDE.md
```

#### Clean Generated Files

```bash
./generate_pdf.sh --clean
```

### Script Options

| Option | Description |
|--------|-------------|
| No arguments | Generate all PDFs |
| `<filename>.md` | Generate specific PDF |
| `--clean` | Remove all generated PDFs and temp files |

---

## Jenkins Automation

### Setup

1. **Create Jenkins Job**

   - Job type: Pipeline
   - Pipeline script from SCM: Git
   - Script path: `Jenkinsfile.docs`

2. **Configure Parameters**

   The pipeline supports these parameters:

   | Parameter | Type | Options | Description |
   |-----------|------|---------|-------------|
   | `GENERATION_MODE` | Choice | all, user_guide, deployment_guide, changelog, readme | Select which docs to generate |
   | `PUBLISH_TO_ARTIFACTORY` | Boolean | true/false | Publish PDFs to Artifactory |

3. **Configure Docker Agent**

   The pipeline uses the `pandoc/latex:latest` Docker image which includes:
   - Pandoc
   - LaTeX distribution
   - All necessary fonts

### Running the Pipeline

#### Option 1: Build with Parameters

1. Go to Jenkins job
2. Click "Build with Parameters"
3. Select `GENERATION_MODE`:
   - `all` - Generate all documentation PDFs
   - `user_guide` - Generate USER_GUIDE.pdf only
   - `deployment_guide` - Generate DEPLOYMENT_GUIDE.pdf only
   - `changelog` - Generate CHANGELOG.pdf only
   - `readme` - Generate README.pdf only
4. Check `PUBLISH_TO_ARTIFACTORY` if needed
5. Click "Build"

#### Option 2: Trigger via Git Webhook

Configure Git webhook to trigger on:
- Push to `main` branch
- Changes in `doc/man/*.md` files

**Example webhook configuration:**

```bash
# GitHub webhook payload URL
https://jenkins.your-domain.com/github-webhook/

# Events to trigger
- Push events
- Pull request events (for /doc/man/* paths)
```

### Pipeline Stages

The Jenkins pipeline executes these stages:

1. **Prepare Environment**
   - Install Node.js
   - Install Mermaid CLI
   - Verify dependencies

2. **Validate Markdown**
   - Check all .md files exist
   - Validate files are not empty

3. **Generate PDFs**
   - Execute `generate_pdf.sh` script
   - Convert Markdown to PDF
   - Process Mermaid diagrams

4. **Verify PDFs**
   - Check PDF files were created
   - Validate file sizes
   - Ensure PDFs are not corrupted

5. **Archive PDFs**
   - Create compressed archive
   - Store artifacts in Jenkins
   - Keep last 10 builds

6. **Publish to Artifactory** (optional)
   - Upload PDFs to artifact repository
   - Version control for documentation

7. **Generate Report**
   - Create build report
   - List all generated PDFs
   - Include file sizes and metadata

### Accessing Build Artifacts

After successful build:

1. Go to build page
2. Click "Build Artifacts"
3. Download:
   - Individual PDFs: `doc/man/pdf/*.pdf`
   - Archive: `uav-swarm-docs-<build_number>.tar.gz`
   - Build log: `doc/man/pdf/generation.log`
   - Report: `doc/man/pdf/report.txt`

---

## Dependencies

### System Requirements

| Tool | Version | Purpose |
|------|---------|---------|
| Pandoc | >= 2.0 | Markdown to PDF conversion |
| LaTeX | Any distribution | PDF rendering engine |
| Node.js | >= 14.0 | For Mermaid CLI |
| Mermaid CLI | >= 10.0 | Diagram conversion |

### Dependency Installation

#### macOS (Complete Setup)

```bash
# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Pandoc
brew install pandoc

# Install LaTeX
brew install basictex

# Install Node.js
brew install node

# Install Mermaid CLI
npm install -g @mermaid-js/mermaid-cli

# Verify installations
pandoc --version
pdflatex --version
node --version
mmdc --version
```

#### Ubuntu/Debian (Complete Setup)

```bash
# Update system
sudo apt-get update

# Install Pandoc
sudo apt-get install -y pandoc

# Install LaTeX
sudo apt-get install -y \
    texlive-latex-base \
    texlive-fonts-recommended \
    texlive-fonts-extra \
    texlive-latex-extra

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Mermaid CLI
sudo npm install -g @mermaid-js/mermaid-cli

# Verify installations
pandoc --version
pdflatex --version
node --version
mmdc --version
```

#### Docker (Recommended for CI/CD)

```bash
# Use pre-built image with all dependencies
docker pull pandoc/latex:latest

# Generate PDFs in Docker
docker run --rm \
    -v $(pwd):/workspace \
    -w /workspace/doc/man \
    pandoc/latex:latest \
    ./generate_pdf.sh
```

---

## Troubleshooting

### Issue: "pandoc: command not found"

**Solution:**
```bash
# Install pandoc
brew install pandoc  # macOS
sudo apt-get install pandoc  # Ubuntu
```

### Issue: "mmdc: command not found"

This is a warning, not an error. Mermaid diagrams will be shown as code blocks.

**Solution (optional):**
```bash
npm install -g @mermaid-js/mermaid-cli
```

### Issue: "pdflatex: command not found"

The script will fallback to wkhtmltopdf, but output quality may be lower.

**Solution:**
```bash
brew install basictex  # macOS
sudo apt-get install texlive-latex-base  # Ubuntu
```

### Issue: PDF generation is very slow

**Causes:**
- Large Mermaid diagrams (conversion takes time)
- Complex LaTeX rendering
- Low system resources

**Solutions:**
1. Generate individual PDFs instead of all at once:
   ```bash
   ./generate_pdf.sh USER_GUIDE.md
   ```

2. Disable Mermaid conversion (faster but no diagrams):
   ```bash
   # Temporarily rename mmdc
   mv $(which mmdc) $(which mmdc).bak
   ./generate_pdf.sh
   mv $(which mmdc).bak $(which mmdc)
   ```

3. Use Docker for better performance:
   ```bash
   docker run --rm -v $(pwd):/workspace -w /workspace/doc/man \
       pandoc/latex ./generate_pdf.sh
   ```

### Issue: Mermaid diagrams not rendering

**Symptoms:**
- Diagrams shown as code blocks in PDF
- Error in generation.log: "Failed to convert Mermaid diagram"

**Solutions:**

1. Check Mermaid CLI installation:
   ```bash
   mmdc --version
   ```

2. Test Mermaid conversion manually:
   ```bash
   cd doc/man/.pdf_tmp
   mmdc -i diagram_1.mmd -o test.png
   ```

3. Check Mermaid syntax:
   - Open .md file in VSCode
   - Use "Markdown Preview Enhanced" extension
   - Verify diagrams render correctly

### Issue: PDF has broken links

**Cause:** Relative links in Markdown don't translate to PDF

**Workaround:** Links to external sections work, but file links don't. This is a Pandoc limitation.

### Issue: Fonts look wrong in PDF

**Solution:**
```bash
# Install additional fonts (Ubuntu)
sudo apt-get install \
    texlive-fonts-recommended \
    texlive-fonts-extra

# Install additional fonts (macOS)
brew install --cask font-libertine
```

### Issue: Jenkins build fails with "Docker not found"

**Solution:**

1. Ensure Docker is installed on Jenkins agent:
   ```bash
   docker --version
   ```

2. Add Jenkins user to docker group:
   ```bash
   sudo usermod -aG docker jenkins
   sudo systemctl restart jenkins
   ```

3. Or use a different agent with Docker installed

---

## Advanced Usage

### Custom PDF Styling

Edit `generate_pdf.sh` and modify pandoc options (line ~165):

```bash
--variable geometry:margin=1.5in      # Wider margins
--variable fontsize=12pt              # Larger font
--variable documentclass=report       # Different document class
--variable mainfont="Times New Roman" # Custom font
```

### Batch Generation Script

Create `generate_all_versions.sh`:

```bash
#!/bin/bash
# Generate PDFs for multiple versions/branches

for branch in main develop release/v1.0; do
    git checkout $branch
    ./doc/man/generate_pdf.sh
    mv doc/man/pdf doc/man/pdf_$branch
done
```

### Integration with GitHub Actions

Create `.github/workflows/docs.yml`:

```yaml
name: Generate Documentation PDFs

on:
  push:
    branches: [ main ]
    paths:
      - 'doc/man/*.md'

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Setup Dependencies
      run: |
        sudo apt-get install -y pandoc texlive-latex-base
        npm install -g @mermaid-js/mermaid-cli

    - name: Generate PDFs
      run: |
        cd doc/man
        ./generate_pdf.sh

    - name: Upload PDFs
      uses: actions/upload-artifact@v3
      with:
        name: documentation-pdfs
        path: doc/man/pdf/*.pdf
```

---

## Support

For issues or questions:

- **Check logs:** `doc/man/pdf/generation.log`
- **GitHub Issues:** Report bugs or request features
- **Email:** docs@your-org.com

---

**Last Updated:** 2026-02-11

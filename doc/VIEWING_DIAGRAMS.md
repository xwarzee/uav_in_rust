# Viewing UML Diagrams

This document explains how to view the UML diagrams in different environments.

## On GitHub

The `ARCHITECTURE.md` file is fully compatible with GitHub! Simply navigate to the document and all diagrams will be displayed as images automatically.

**Direct link**: `doc/ARCHITECTURE.md`

All diagrams are rendered as PNG images that GitHub displays natively. The PlantUML source code is available in collapsible sections if you need to view or modify it.

## In VS Code

### Using Markdown Preview

1. Open `doc/ARCHITECTURE.md`
2. Press `Cmd+Shift+V` (macOS) or `Ctrl+Shift+V` (Windows/Linux)
3. Images will be displayed inline

### Using PlantUML Extension (for .puml files)

1. Install the "PlantUML" extension by jebbs
2. Open any `.puml` file in the `doc/` folder
3. Press `Alt+D` (or `Option+D` on macOS) to preview

The workspace is already configured to use the PlantUML server at `https://www.plantuml.com/plantuml`.

## Regenerating Images

If you modify any `.puml` file, regenerate the PNG images:

```bash
# Download PlantUML JAR (if not already done)
curl -L -o plantuml.jar https://github.com/plantuml/plantuml/releases/download/v1.2024.8/plantuml-1.2024.8.jar

# Generate images
java -jar plantuml.jar -tpng -o images doc/*.puml
```

Images will be saved in `doc/images/`.

## Online Viewing

You can also view individual diagrams online:

1. Copy the content of any `.puml` file
2. Go to [PlantUML Web Server](http://www.plantuml.com/plantuml/uml/)
3. Paste the content
4. View the rendered diagram

## Available Diagrams

- **Class Diagram** (`class_diagram.puml`) - System structure and relationships
- **Module Diagram** (`module_diagram.puml`) - Module dependencies
- **Mission Execution Sequence** (`sequence_mission_execution.puml`) - Mission flow
- **Formation Change Sequence** (`sequence_formation_change.puml`) - Formation updates
- **Drone State Diagram** (`state_diagram.puml`) - State machine
- **Simulation Activity** (`activity_diagram_simulation.puml`) - Application flow

## Image Formats

Current images are in PNG format. To generate SVG (scalable):

```bash
java -jar plantuml.jar -tsvg -o images doc/*.puml
```

SVG files are better for zooming but may not display on all platforms.

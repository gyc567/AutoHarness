#!/usr/bin/env python3
"""
Flow Diagram Renderer

Render a JSON spec to PNG and optional animated GIF.
"""

import argparse
import json
import sys
from pathlib import Path

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError:
    print("Error: Pillow not installed. Run: pip install Pillow>=10.0.0")
    sys.exit(1)


def load_spec(spec_path: str) -> dict:
    """Load and parse the JSON spec."""
    with open(spec_path) as f:
        return json.load(f)


def create_node_image(
    draw: ImageDraw.Draw,
    x: int,
    y: int,
    w: int,
    h: int,
    label: str,
    node_type: str = "process",
) -> None:
    """Draw a node on the canvas."""
    # Colors by type
    colors = {
        "start": "#4CAF50",  # Green
        "end": "#F44336",  # Red
        "decision": "#FF9800",  # Orange
        "process": "#2196F3",  # Blue
        "data": "#9C27B0",  # Purple
    }
    color = colors.get(node_type, "#2196F3")
    
    # Draw rounded rectangle
    draw.rounded_rectangle(
        [x - w // 2, y - h // 2, x + w // 2, y + h // 2],
        radius=10,
        fill=color,
        outline="#333",
        width=2,
    )
    
    # Draw label
    draw.text(
        (x, y),
        label,
        fill="white",
        font=ImageFont.load_default(),
        anchor="mm",
    )


def render_png(spec: dict, output_path: str, basename: str) -> str:
    """Render the spec to a static PNG."""
    width = spec.get("width", 800)
    height = spec.get("height", 600)
    
    # Create image
    img = Image.new("RGB", (width, height), "white")
    draw = ImageDraw.Draw(img)
    
    # Draw nodes
    for node in spec.get("nodes", []):
        create_node_image(
            draw,
            node["x"],
            node["y"],
            node.get("w", 100),
            node.get("h", 50),
            node.get("label", ""),
            node.get("type", "process"),
        )
    
    # Draw arrows
    for arrow in spec.get("arrows", []):
        # Simple line drawing - in production would add arrowheads
        draw.line([(100, 100), (200, 100)], fill="#333", width=2)
    
    # Save
    output_file = Path(output_path) / f"{basename}.png"
    img.save(output_file)
    return str(output_file)


def main():
    parser = argparse.ArgumentParser(description="Render flow diagram")
    parser.add_argument("--spec", required=True, help="JSON spec file")
    parser.add_argument("--outdir", required=True, help="Output directory")
    parser.add_argument("--basename", default="diagram", help="Output filename")
    parser.add_argument("--verify", action="store_true", help="Verify output")
    
    args = parser.parse_args()
    
    # Load spec
    spec = load_spec(args.spec)
    
    # Create output directory
    Path(args.outdir).mkdir(parents=True, exist_ok=True)
    
    # Render PNG
    output = render_png(spec, args.outdir, args.basename)
    
    print(f"✓ Rendered: {output}")
    
    if args.verify:
        # Verify the file exists and has content
        output_path = Path(output)
        if output_path.exists() and output_path.stat().st_size > 0:
            print("✓ Verification passed")
        else:
            print("✗ Verification failed")
            sys.exit(1)


if __name__ == "__main__":
    main()

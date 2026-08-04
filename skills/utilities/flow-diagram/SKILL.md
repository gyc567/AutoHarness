---
name: flow-diagram
description: >
  Create animated flow diagrams from articles, workflow notes, architecture sketches, or process
  descriptions using a JSON specification and a local Python/Pillow renderer. Use when the user wants
  to turn source material into a static PNG + animated GIF diagram.
user_invocable: true
---

# Visual Flow Diagram

Use this skill when a user wants to turn source material into a clear animated flow diagram.

## When to use

- Turn architecture docs into diagrams
- Create workflow visualizations
- Make process explanations clearer

## Workflow

1. **Read source material** — Identify inputs, outputs, actors, modules, tools, stores, and feedback loops

2. **Create JSON spec** — Define the diagram structure:
   ```json
   {
     "width": 800,
     "height": 600,
     "nodes": [
       {"id": "start", "type": "start", "x": 100, "y": 300, "label": "Start"}
     ],
     "arrows": [
       {"from": "start", "to": "process", "label": "action"}
     ],
     "animation": {
       "pulses": ["start", "process"]
     }
   }
   ```

3. **Render the diagram**:
   ```bash
   python3 scripts/render_diagram.py --spec /path/to/spec.json --outdir /path/to/output --basename diagram --verify
   ```

4. **Inspect output** — Check readability, overlaps, and animation

## Output

- Static PNG diagram
- Animated GIF (optional)
- JSON spec for future edits

## Style Guide

- Clean technical diagram style
- Use colors to separate roles:
  - Blue: core process
  - Green: active loops / success
  - Purple: shared layers
  - Red: friction / warnings
- Short, concrete node labels
- Rounded routed arrows

## Requirements

- Python 3.10+
- Pillow 10+

```bash
pip install -r requirements.txt
```

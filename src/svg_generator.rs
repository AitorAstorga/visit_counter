/// Generates an SVG counter image.
///
/// * `label` - The label to display on the left side.
/// * `count` - The counter value to display on the right side.
/// * `css`   - The CSS to embed in the SVG.
pub fn generate_svg(label: &str, count: u64, css: &str) -> String {
    format!(
      r##"<?xml version="1.0" encoding="UTF-8"?>
      <svg xmlns="http://www.w3.org/2000/svg" width="150" height="20">
        <style type="text/css">
          {css}
        </style>
        <defs>
          <linearGradient id="grad" x2="0" y2="100%">
            <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
            <stop offset="1" stop-opacity=".1"/>
          </linearGradient>
          <mask id="mask">
            <rect width="150" height="20" rx="3" fill="#fff"/>
          </mask>
        </defs>
        <g mask="url(#mask)">
          <!-- Left section: label background -->
          <rect width="100" height="20" fill="var(--background-body)"/>
          <!-- Right section: counter background -->
          <rect x="100" width="50" height="20" fill="var(--accent-primary)"/>
          <!-- Overlay gradient for subtle effect -->
          <rect width="150" height="20" fill="url(#grad)"/>
        </g>
        <g fill="#fff" text-anchor="middle" font-family="'Metrophobic', 'Comfortaa', sans-serif" font-size="11">
          <!-- Label text with a subtle drop shadow -->
          <text x="50" y="15" fill="#010101" fill-opacity=".3">{label}</text>
          <text x="50" y="14">{label}</text>
          <!-- Counter text with a subtle drop shadow -->
          <text x="125" y="15" fill="#010101" fill-opacity=".3">{count}</text>
          <text x="125" y="14">{count}</text>
        </g>
      </svg>"##,
      css = css,
      label = label,
      count = count
    )
}

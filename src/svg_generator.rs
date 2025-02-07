use crate::models::SvgOptions;

/// Generates an SVG counter image.
/// * `label` - The label to display on the left side.
/// * `count` - The counter value to display on the right side.
/// * `css`   - The CSS to embed in the SVG.
pub fn generate_svg(label: &str, count: u64, css: &str, width: u32, height: u32) -> String {
  format!(
r##"<?xml version="1.0" encoding="UTF-8"?>
<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg" class="svg-counter">
<style type="text/css"><![CDATA[
{css}
]]></style>
<defs>
  <linearGradient id="grad" x2="0" y2="100%">
    <stop offset="0" stop-color="var(--grad-stop1-color)" stop-opacity="var(--grad-stop1-opacity)"/>
    <stop offset="1" stop-opacity="var(--grad-stop2-opacity)"/>
  </linearGradient>
  <mask id="mask">
    <rect class="mask-rect" fill="#fff"/>
  </mask>
</defs>
<g mask="url(#mask)">
  <rect class="left-rect"/>
  <rect class="right-rect"/>
  <rect class="overlay-rect" fill="url(#grad)"/>
</g>
<g class="text-group">
  <text class="label-shadow">{label}</text>
  <text class="label">{label}</text>
  <text class="count-shadow">{count}</text>
  <text class="count">{count}</text>
</g>
</svg>"##,
      width = width,
      height = height,
      css = css,
      label = label,
      count = count
  )
}

// Build custom CSS if parameters are provided
pub fn build_custom_css(options: Option<SvgOptions>) -> String {
  let mut custom_css = String::new();

  if let Some(opts) = options {
      custom_css.push_str(":root {\n");

      // A helper that pushes a CSS property if the option is Some.
      fn push_prop<T, F>(css: &mut String, name: &str, value: Option<T>, formatter: F)
      where
          F: FnOnce(T) -> String,
      {
          if let Some(val) = value {
              css.push_str(&format!("  {}: {};\n", name, formatter(val)));
          }
      }

      // Formatter function for numeric values: append "px".
      fn to_px(v: u32) -> String {
          format!("{}px", v)
      }

      // Formatter that returns the string unchanged.
      let identity = |v: String| v;

      // Formatter that normalizes a color by ensuring it starts with '#'.
      let normalize_color = |color: String| {
          if color.starts_with('#') {
              color
          } else {
              format!("#{}", color)
          }
      };

      // SVG Dimensions
      push_prop(&mut custom_css, "--width", opts.width, to_px);
      push_prop(&mut custom_css, "--height", opts.height, to_px);
      push_prop(&mut custom_css, "--label-width", opts.label_width, to_px);
      push_prop(&mut custom_css, "--counter-width", opts.counter_width, to_px);
      push_prop(&mut custom_css, "--radius", opts.radius, to_px);

      // Gradient Settings
      push_prop(&mut custom_css, "--grad-stop1-color", opts.grad_stop1_color, normalize_color);
      push_prop(&mut custom_css, "--grad-stop1-opacity", opts.grad_stop1_opacity, |v| v.to_string());
      push_prop(&mut custom_css, "--grad-stop2-opacity", opts.grad_stop2_opacity, |v| v.to_string());

      // Text Settings
      push_prop(&mut custom_css, "--font-family", opts.font_family, identity);
      push_prop(&mut custom_css, "--font-size", opts.font_size, to_px);
      push_prop(&mut custom_css, "--label-offset-x", opts.label_offset_x, to_px);
      push_prop(&mut custom_css, "--label-offset-y", opts.label_offset_y, to_px);
      push_prop(&mut custom_css, "--counter-offset-x", opts.counter_offset_x, to_px);
      push_prop(&mut custom_css, "--counter-offset-y", opts.counter_offset_y, to_px);
      push_prop(&mut custom_css, "--shadow-fill", opts.shadow_fill, normalize_color);
      push_prop(&mut custom_css, "--shadow-opacity", opts.shadow_opacity, |v| v.to_string());

      // Color Settings
      push_prop(&mut custom_css, "--background-label", opts.background_label, normalize_color,);
      push_prop(&mut custom_css, "--background-counter", opts.background_counter, normalize_color);
      push_prop(&mut custom_css, "--label-color", opts.label_color, normalize_color);
      push_prop(&mut custom_css, "--counter-color", opts.counter_color, normalize_color,);

      custom_css.push_str("}\n");

      // Append any additional CSS from the `style` field.
      if let Some(style) = opts.style {
          custom_css.push_str(&style);
      }
  }

  custom_css
}
// backend_visit_counter/src/svg_generator.rs
use crate::models::SvgOptions;

/// Generates an SVG counter image.
/// * `label` - The label to display on the left side.
/// * `count` - The counter value to display on the right side.
/// * `css`   - The CSS to embed in the SVG.
/// * `options` - Optional parameters for customization.
pub fn generate_svg(label: &str, count: u64, css: &str, width: u32, height: u32, options: Option<&crate::models::SvgOptions>) -> String {
  let has_border = options
    .and_then(|opts| opts.border_width)
    .map(|w| w > 0)
    .unwrap_or(false);

  let has_logo = options
    .and_then(|opts| opts.logo_url.as_ref())
    .map(|url| !url.is_empty())
    .unwrap_or(false);

  let logo_element = if has_logo {
    let logo_url = options.unwrap().logo_url.as_ref().unwrap();

    // Calculate logo dimensions based on height with padding
    let logo_size = if height > 20 {
        (height as f32 * 0.7).round() as u32
    } else {
        (height as f32 * 0.8).round() as u32
    };
    let logo_margin = (height - logo_size) / 2;

    // Logo will be positioned using CSS variables (--logo-offset-x) like text
    format!("<rect class=\"logo-rect\"/><image href=\"{}\" xlink:href=\"{}\" class=\"logo-image\" width=\"{}\" height=\"{}\" preserveAspectRatio=\"xMidYMid meet\"/>", logo_url, logo_url, logo_size, logo_size)
  } else {
    String::new()
  };

  let border_element = if has_border {
    let border_width = options.and_then(|opts| opts.border_width).unwrap_or(1) as f32;
    let half_border = border_width / 2.0;
    let rect_width = width as f32 - border_width;
    let rect_height = height as f32 - border_width;

    format!("<rect class=\"border-rect\" width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\"/>",
            rect_width, rect_height, half_border, half_border)
  } else {
    String::new()
  };

  format!(
r##"<?xml version="1.0" encoding="UTF-8"?>
<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" class="svg-counter">
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
  {logo_element}
</g>
{border_element}
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
      count = count,
      border_element = border_element,
      logo_element = logo_element
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

      // SVG Dimensions with dynamic width distribution
      if let Some(width) = opts.width {
          custom_css.push_str(&format!("  --width: {}px;\n", width));

          // Calculate section widths based on element positions
          if opts.label_width.is_none() && opts.counter_width.is_none() {
              let positions = opts.element_positions.as_ref()
                  .map(|s| s.as_str())
                  .unwrap_or("label,logo,counter");
              let elements: Vec<&str> = positions.split(',').map(|s| s.trim()).collect();

              // Calculate widths: if logo present, make it smaller, distribute rest between label/counter
              let has_logo = elements.contains(&"logo") && opts.logo_url.as_ref().map_or(false, |url| !url.is_empty());
              let logo_width = if has_logo { opts.logo_width.unwrap_or(30) } else { 0 };

              let remaining_width = width - logo_width;
              let text_sections = if has_logo { 2 } else { 2 }; // label + counter
              let section_width = remaining_width / text_sections;

              // Set section widths and positions based on element order
              let mut current_x = 0u32;
              for (i, element) in elements.iter().enumerate() {
                  match *element {
                      "label" => {
                          let section_w = if has_logo { section_width } else { (width as f32 * 0.667).round() as u32 };
                          custom_css.push_str(&format!("  --label-width: {}px;\n", section_w));
                          custom_css.push_str(&format!("  --label-offset-x: {}px;\n", current_x + section_w / 2));
                          current_x += section_w;
                      },
                      "logo" => {
                          if has_logo {
                              custom_css.push_str(&format!("  --logo-width: {}px;\n", logo_width));
                              custom_css.push_str(&format!("  --logo-offset-x: {}px;\n", current_x + logo_width / 2));
                              current_x += logo_width;
                          }
                      },
                      "counter" => {
                          let section_w = if has_logo { section_width } else { width - (width as f32 * 0.667).round() as u32 };
                          custom_css.push_str(&format!("  --counter-width: {}px;\n", section_w));
                          custom_css.push_str(&format!("  --counter-offset-x: {}px;\n", current_x + section_w / 2));
                          current_x += section_w;
                      },
                      _ => {}
                  }
              }
          } else {
              // Use explicit widths if given
              push_prop(&mut custom_css, "--label-width", opts.label_width, to_px);
              push_prop(&mut custom_css, "--counter-width", opts.counter_width, to_px);
          }
      } else {
          push_prop(&mut custom_css, "--width", opts.width, to_px);
          push_prop(&mut custom_css, "--label-width", opts.label_width, to_px);
          push_prop(&mut custom_css, "--counter-width", opts.counter_width, to_px);
      }

      push_prop(&mut custom_css, "--height", opts.height, to_px);
      // Handle main radius (affects the inner badge shape)
      if let Some(radius) = opts.radius {
          custom_css.push_str(&format!("  --radius: {}px;\n", radius));
          // Only affect the mask rect (inner badge shape), not border
          custom_css.push_str(&format!(".mask-rect {{ rx: {}px; ry: {}px; }}\n", radius, radius));
      }

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
      let has_label_color = opts.label_color.is_some();
      let has_counter_color = opts.counter_color.is_some();
      let has_background_label = opts.background_label.is_some();
      let has_background_counter = opts.background_counter.is_some();

      push_prop(&mut custom_css, "--background-label", opts.background_label, normalize_color,);
      push_prop(&mut custom_css, "--background-counter", opts.background_counter, normalize_color);
      push_prop(&mut custom_css, "--label-color", opts.label_color, normalize_color);
      push_prop(&mut custom_css, "--counter-color", opts.counter_color, normalize_color,);

      // Specific label/counter colors
      // Text color and background color are deprecated but kept for backwards compatibility if specific colors aren't set
      if let Some(text_color) = opts.text_color {
          let color = normalize_color(text_color);
          // Only set if specific colors aren't already set
          if !has_label_color {
              custom_css.push_str(&format!("  --label-color: {};\n", color));
          }
          if !has_counter_color {
              custom_css.push_str(&format!("  --counter-color: {};\n", color));
          }
      }

      if let Some(background_color) = opts.background_color {
          let color = normalize_color(background_color);
          // Only set if specific colors aren't already set
          if !has_background_label {
              custom_css.push_str(&format!("  --background-label: {};\n", color));
          }
          if !has_background_counter {
              custom_css.push_str(&format!("  --background-counter: {};\n", color));
          }
      }

      custom_css.push_str("}\n");

      // Add CSS for logo section positioning and styling
      if opts.logo_url.as_ref().map_or(false, |url| !url.is_empty()) {
          custom_css.push_str(&format!(r#"
.logo-rect {{
  width: var(--logo-width, 30px);
  height: var(--height);
  fill: var(--background-logo, transparent);
  transform: translateX(calc(var(--logo-offset-x, 50px) - var(--logo-width, 30px) / 2));
}}
.logo-image {{
  transform: translate(calc(var(--logo-offset-x, 50px) - {}px), {}px);
}}
"#, opts.logo_width.unwrap_or(30) / 2, (opts.height.unwrap_or(20) - if opts.height.unwrap_or(20) > 20 { (opts.height.unwrap_or(20) as f32 * 0.7).round() as u32 } else { (opts.height.unwrap_or(20) as f32 * 0.8).round() as u32 }) / 2));
      }

      // Handle border radius
      // The inner badge should have a smaller radius to stay inside the border
      let has_border = opts.border_width.map(|w| w > 0).unwrap_or(false);
      if let Some(border_radius) = opts.border_radius {
          // User specified border radius
          custom_css.push_str(&format!(".border-rect {{ rx: {}px; ry: {}px; }}\n", border_radius, border_radius));

          // Set inner badge radius
          // Should be smaller but not by the full border width
          let border_width = opts.border_width.unwrap_or(1) as f32;
          let inner_radius = if border_radius as f32 > border_width * 0.5 {
              ((border_radius as f32) - (border_width * 0.5)).round() as u32
          } else {
              0
          };
          custom_css.push_str(&format!(".mask-rect {{ rx: {}px; ry: {}px; }}\n", inner_radius, inner_radius));
      } else if has_border {
          // Default border radius when border existsbut no radius specified
          custom_css.push_str(&format!(".border-rect {{ rx: 3px; ry: 3px; }}\n"));
          let border_width = opts.border_width.unwrap_or(1) as f32;
          let inner_radius = if 3.0 > border_width * 0.5 {
              (3.0 - (border_width * 0.5)).round() as u32
          } else {
              0
          };
          custom_css.push_str(&format!(".mask-rect {{ rx: {}px; ry: {}px; }}\n", inner_radius, inner_radius));
      }

      // Add CSS rules for font weight
      if let Some(ref font_weight) = opts.font_weight {
          custom_css.push_str(&format!(":root {{ --font-weight: {}; }}\n", font_weight));
          custom_css.push_str(&format!(".text-group {{ font-weight: {} !important; }}\n", font_weight));
      }

      // Fix border rendering by expanding SVG viewBox and positioning
      if let Some(border_width) = opts.border_width {
          if border_width > 0 {
              let border_color = opts.border_color.as_ref()
                  .map(|c| normalize_color(c.clone()))
                  .unwrap_or_else(|| "#cccccc".to_string());

              // Add border as a rect element that fits within the SVG
              // Don't set rx here because it will be set separately based on border_radius option
              custom_css.push_str(&format!(
                  ".border-rect {{ fill: none; stroke: {}; stroke-width: {}; }}\n",
                  border_color, border_width
              ));
          }
      }

      // Dynamic text positioning based on height (for propper centering)
      if let Some(height) = opts.height {
          if height != 20 {
              // Center the text vertically by calculating the base position
              // SVG text baseline needs to be positioned at the center + a small adjustment
              let font_size = opts.font_size.unwrap_or(11) as f32;
              // For better centering: center of badge + slight upward adjustment for baseline
              let text_center_y = (height as f32 / 2.0) + (font_size * 0.35);
              let label_y = text_center_y.round() as u32;
              let counter_y = text_center_y.round() as u32;
              custom_css.push_str(&format!(":root {{ --label-offset-y: {}px; --counter-offset-y: {}px; }}\n", label_y, counter_y));
          }
      }

      // Append any additional CSS from the `style` field.
      if let Some(style) = opts.style {
          custom_css.push_str(&style);
      }
  }

  custom_css
}
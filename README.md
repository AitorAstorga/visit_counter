[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
 <a href="https://github.com/AitorAstorga/visit_counter">
 </a>

 <h1 align="center">Visit Counter API</h1>
 <p align="center"> <img 
    src="https://visitcounter.aichan.ovh/counter/YOUR_PAGE_NAME/svg?label=Example%20Visits" height=20
    alt="Visit Counter" /> </p>

 <p align="center">
 A self‑hostable API service in Rust for tracking website visits and generating customizable SVG counters.
 <br />
 <br />
 <a href="https://github.com/AitorAstorga/visit_counter">View Demo</a>
 ·
 <a href="https://github.com/AitorAstorga/visit_counter/issues">Report Bug</a>
 ·
 <a href="https://github.com/AitorAstorga/visit_counter/issues">Request Feature</a>
 </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
 <summary>Table of Contents</summary>
 <ol>
 <li><a href="#about-the-project">About The Project</a></li>
   <ul>
      <li><a href="#usage">Usage</a></li>
   </ul>
   <ul>
      <li><a href="#built-with">Built With</a></li>
   </ul>
 </li>
 <li>
 <a href="#project-structure">Project Structure</a>
 <ul>
 <li><a href="#modules">Modules</a></li>
 </ul>
 </li>
 <li><a href="#getting-started">Getting Started</a></li>
 <li><a href="#testing-the-api">Testing the API</a></li>
 <li><a href="#docker-deployment">Docker Deployment</a></li>
 <li><a href="#contributing">Contributing</a></li>
 <li><a href="#license">License</a></li>
 <li><a href="#contact">Contact</a></li>
 </ol>
</details>

## About The Project

The Visit Counter API is a lightweight, self‑hostable backend service built in Rust using Rocket. It tracks website visits and dynamically generates SVG counters with customizable styling. This project is modularized as follows:

- **SVG Generator Module**: All SVG-generation logic is contained in `src/svg_generator.rs`.
- **External CSS**: Styling is maintained in `assets/style.css`, allowing easy customization of colors, fonts, and borders.
- **API Endpoints**: Endpoints are provided to get, increment, and set counter values, along with an SVG endpoint for embedding counters into webpages.
- **Persistent Storage**: Counter data is stored in `counters.json` via a file-based persistence module (`src/persistent_counter.rs`), ensuring counters are preserved across API restarts.

### Usage

If you simply want to utilize my API, you can simply add an `img` tag to your site:
```html
<img
    src="https://visitcounter.aichan.ovh/counter/YOUR_PAGE_NAME/svg?label=YOUR_TEXT&color=YOUR_COLOR&style=font-weight:bold;"
    alt="Visit Counter" />
```

#### Parameters
- Page name: Don't forget to set it `https://visitcounter.aichan.ovh/counter/YOUR_PAGE_NAME...`
- `label`: The text shown to the left.
- `style`: Directly embed CSS in here. Something like `":root { --background-counter: red; }"` would work.

> [!TIP]
> If you intend to use this in GitHub, make sure you encode all spaces with `%20`. [HTML URL Encoding Reference](https://www.w3schools.com/tags//ref_urlencode.asp)

#### Customizing the SVG Appearance

The style is controlled with the CSS variables defined `assets/style.css`. These constants can also be modified to adjust the appearance:
- **SVG Dimensions**:
   - `width`: Overall width of the SVG (default: 150).
   - `height`: Overall height of the SVG (default: 20).
   - `label_width`: Width of the left section (label background) (default: 100).
   - `counter_width`: Width of the right section (counter background) (default: 50).
   - `radius`: Border radius for rounded corners (default: 3).

- **Gradient Settings**:
   - `grad_stop1_color`: Color of the first gradient stop (default: #bbb).
   - `grad_stop1_opacity`: Opacity of the first gradient stop (default: 0.1).
   - `grad_stop2_opacity`: Opacity of the second gradient stop (default: 0.1).

- **Text Settings**:
   - `font_family`: Font family used for the counter text (default: 'Metrophobic', 'Comfortaa', sans-serif).
   - `font_size`: Font size for the text (default: 11).
   - `label_offset_x`: X-coordinate for the label text (default: 50).
   - `label_offset_y`: Y-coordinate for the label text (default: 15).
   - `counter_offset_x`: X-coordinate for the counter text (default: 125).
   - `counter_offset_y`: Y-coordinate for the counter text (default: 15).
   - `shadow_fill`: Color used for the text drop shadow (default: #010101).
   - `shadow_opacity`: Opacity of the text drop shadow (default: 0.3).

- **Color Settings (NyakoTech Inspired)**:
   - `background_label`: Background for the label section (default: #18181b).
   - `background_counter`: Background for the counter section (default: #DC26B6).
   - `label_color`: Color of the label text (default:#fff).
   - `counter_color`: Color of the counter text (default:#fff).

<p align="right">(<a href="#about-the-project">back to top</a>)</p>

### Built With
- ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
- ![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

## Project Structure

The project is organized as follows:

```
visit_counter/
├── Cargo.toml
├── .env # Environment variables file (optional)
├── assets/
│ └── style.css # External CSS for SVG styling
└── src/
 ├── main.rs # Main application with API endpoints
 ├── models.rs # Data structures
 ├── svg_generator.rs# Module for generating SVG content
 └── persistent_counter.rs # Module for file-based persistent storage
```

### Modules

- models.rs: Contains data structures for counters and SVG options.
- svg_generator.rs: Provides the `generate_svg` function that creates the SVG output.
- main.rs: Defines API endpoints, loads environment variables, and integrates all modules.

<p align="right">(<a href="#project-structure">back to top</a>)</p>

## Getting Started

Follow these instructions to set up a local instance of the Visit Counter API.

### Prerequisites

- Rust installed
- Cargo (bundled with Rust)
- (Optional) Docker if you plan to deploy in a container

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/AitorAstorga/visit_counter.git
   cd visit_counter
   ```

2. (Optional) Create a `.env` file in the project root:
   This file should define your API key:
   ```env
   API_KEY=your_secret_api_key_here
   ```

3. Run the application locally:
   ```bash
   export API_KEY=your_secret_api_key_here # if not using `.env`
   cargo run
   ```

By default, Rocket runs on `localhost:8000`.

<p align="right">(<a href="#getting-started">back to top</a>)</p>

## Testing the API

You can test the endpoints using your browser, curl, or any HTTP client.

### API Endpoints

- GET Counter Value (JSON):
   ```bash
   curl http://localhost:8000/api/counter/test
   ```

- Increment Counter (JSON):
   ```bash
   curl -X POST http://localhost:8000/api/counter/test/increment
   ```

- Set Counter Value (JSON, Requires API Key):
   ```bash
   curl -X PUT http://localhost:8000/api/counter/test \
   -H "Content-Type: application/json" \
   -H "x-api-key: your_secret_api_key_here" \
   -d '{"count": 123}'
   ```

- SVG Counter Endpoint:
 Open your browser or use curl:
   ```bash
   curl "http://localhost:8000/counter/test/svg?label=Page%20Views&color=ff5733&style=font-weight:bold;"
   ```
   This returns an SVG image with your counter, which you can embed using an `<img>` tag.

<p align="right">(<a href="#testing-the-api">back to top</a>)</p>

## Docker Deployment

If you prefer containerized deployment, you can use Docker.

### Dockerfile

A sample
```dockerfile
# Use an official Rust image as the builder.
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY assets/ ./assets/

RUN cargo build --release

# Use a minimal image for the final binary.
FROM debian:buster-slim
COPY --from=builder /app/target/release/visit_counter /usr/local/bin/
EXPOSE 8000
CMD ["visit_counter"]
```

### docker-compose.yml

A sample
```yaml
version: "3"
services:
  counter:
    container_name: visit-counter
    image: ghcr.io/aitorastorga/aichan-visit-counter:latest
    ports:
      - "8000:8000"
    environment:
      ROCKET_ADDRESS: "0.0.0.0"
      ROCKET_PORT: "8000"
      API_KEY: "your_secret_api_key_here"
    volumes:
      - /PATH_TO_YOUR_DATA:/data
```

<p align="right">(<a href="#docker-deployment">back to top</a>)</p>

## Contributing

Contributions are welcome! Please fork the repository, make your changes, and open a pull request.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#contributing">back to top</a>)</p>

## License

Distributed under the European Union Public License v1.2. See `LICENSE` for more information.

<p align="right">(<a href="#license">back to top</a>)</p>

## Contact

Aitor Astorga Saez de Vicuña - a.astorga.sdv@protonmail.com

Project Link: [https://github.com/AitorAstorga/visit_counter](https://github.com/AitorAstorga/visit_counter)

<p align="right">(<a href="#contact">back to top</a>)</p>

## Acknowledgments

This is based on [Anton Komarev's](https://komarev.com/anton) [github-profile-views-counter](https://github.com/antonkomarev/github-profile-views-counter), but made in Rust.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
[contributors-shield]: https://img.shields.io/github/contributors/AitorAstorga/visit_counter.svg?style=for-the-badge
[contributors-url]: https://github.com/AitorAstorga/visit_counter/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/AitorAstorga/visit_counter.svg?style=for-the-badge
[forks-url]: https://github.com/AitorAstorga/visit_counter/network/members
[stars-shield]: https://img.shields.io/github/stars/AitorAstorga/visit_counter.svg?style=for-the-badge
[stars-url]: https://github.com/AitorAstorga/visit_counter/stargazers
[issues-shield]: https://img.shields.io/github/issues/AitorAstorga/visit_counter.svg?style=for-the-badge
[issues-url]: https://github.com/AitorAstorga/visit_counter/issues
[license-shield]: https://img.shields.io/github/license/AitorAstorga/visit_counter.svg?style=for-the-badge
[license-url]: https://github.com/AitorAstorga/visit_counter/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/aitor-astorga-saez-de-vicuña

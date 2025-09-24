<!-- PROJECT LOGO -->
<br />
<div align="center">
 <a href="https://git.prisma.moe/aichan/visit_counter">
 </a>

 <h1 align="center">Visit Counter API</h1>
 <p align="center"> <img
    src="https://visitcounter.aichan.ovh/counter/visit_counter_project/svg?label=Project%20Visits" height=20
    alt="Visit Counter" /> </p>

 <p align="center">
 A self‑hostable API service in Rust for tracking website visits and generating customizable SVG counters with a Yew WebAssembly frontend.
 <br />
 <br />
 <a href="https://git.prisma.moe/aichan/visit_counter">View Demo</a>
 ·
 <a href="https://git.prisma.moe/aichan/visit_counter/issues">Report Bug</a>
 ·
 <a href="https://git.prisma.moe/aichan/visit_counter/issues">Request Feature</a>
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
 <li><a href="#testing">Testing</a></li>
 <li><a href="#docker-deployment">Docker Deployment</a></li>
 <li><a href="#contributing">Contributing</a></li>
 <li><a href="#license">License</a></li>
 <li><a href="#contact">Contact</a></li>
 </ol>
</details>

## About The Project

The Visit Counter API is a lightweight, self‑hostable service built in Rust that tracks website visits and generates customizable SVG counters. The backend is built with Rocket. It features a web interface built with Yew WebAssembly for badge management and administration.

- **SVG Generator Module**: All SVG-generation logic is contained in `backend_visit_counter/src/svg_generator.rs`.
- **Web Interface**: Yew WebAssembly frontend for badge management and administration.
- **API Endpoints**: RESTful API for counter operations and admin management.
- **Persistent Storage**: JSON-based storage with authentication for administrative operations.

### Usage

#### Web Interface
Visit [**https://visitcounter.aichan.ovh/**](https://visitcounter.aichan.ovh/) to use the interactive badge generator interface. This web application allows you to customize your visit counter badge with a live preview and automatically generates the HTML code for you.

#### Direct API Usage
If you prefer to use the API directly, you can add an `img` tag to your site:
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

<p align="right">(<a href="#about-the-project">back to top</a>)</p>

### Built With
![Rust](static/badges/rust-badge.svg) ![WebAssembly](static/badges/webassembly-badge.svg) [![Yew](static/badges/yew-badge.svg)](#) [![Rocket](static/badges/rocket-badge.svg)](#) ![Docker](static/badges/docker-badge.svg)

## Project Structure

The project is organized as follows:

```
visit_counter/
├── backend_visit_counter/    # Rocket backend API
│   ├── src/
│   │   ├── main.rs
│   │   ├── models.rs
│   │   ├── svg_generator.rs
│   │   └── persistent_counter.rs
│   └── Cargo.toml
├── frontend_visit_counter/   # Yew WebAssembly frontend
│   ├── src/
│   │   ├── main.rs
│   │   ├── app.rs
│   │   ├── components/
│   │   ├── services.rs
│   │   └── types.rs
│   ├── index.html
│   └── Cargo.toml
├── static/assets/           # Shared CSS and assets
│   ├── style.css
│   ├── minimal-icons.css
│   └── badges/
└── data/                   # Persistent data storage
```

### Modules

- **Backend**: Rocket-based REST API with SVG generation and persistent storage
- **Frontend**: Yew WebAssembly application for badge management and admin interface
- **Static Assets**: Shared CSS, icons, and badges stored locally

<p align="right">(<a href="#project-structure">back to top</a>)</p>

## Getting Started

Follow these instructions to set up a local instance of the Visit Counter API.

### Prerequisites

- Rust installed
- Cargo (bundled with Rust)
- Trunk (for frontend development): `cargo install --locked trunk`
- (Optional) Docker for containerized deployment

### Development Setup

#### Frontend Development
```bash
cd frontend_visit_counter
trunk serve
```
This starts the frontend development server at `http://localhost:8080`

#### Backend Development
```bash
cd backend_visit_counter
cargo run
```
This starts the API server at `http://localhost:8000`

#### Production Build
Frontend:
```bash
cd frontend_visit_counter
trunk build --release
```

Backend:
```bash
cd backend_visit_counter
cargo build --release
```

<p align="right">(<a href="#getting-started">back to top</a>)</p>

## Testing

**Recommended**: Use the development Docker Compose setup for testing:

```bash
docker-compose -f docker-compose.dev.yml up
```

This will start both the backend and frontend services in a development environment with hot reloading and proper networking.

### Manual API Testing

You can also test the endpoints using curl or any HTTP client:

- **GET Counter Value (JSON)**:
   ```bash
   curl http://localhost:8000/api/counter/test
   ```

- **Increment Counter (JSON)**:
   ```bash
   curl -X POST http://localhost:8000/api/counter/test/increment
   ```

- **SVG Counter Endpoint**:
   ```bash
   curl "http://localhost:8000/counter/test/svg?label=Page%20Views&color=ff5733"
   ```

<p align="right">(<a href="#testing">back to top</a>)</p>

## Docker Deployment

### Using Docker Compose (Recommended)

Create a `docker-compose.yml` file:
```yaml
services:
  visit_counter:
    container_name: visit-counter
    image: git.prisma.moe/aichan/visit_counter:latest
    ports:
      - "8000:8000"
    environment:
      ROCKET_ADDRESS: "0.0.0.0"
      ROCKET_PORT: "8000"
      API_KEY: "your_secret_api_key_here"
      ADMIN_PASSWORD: "your_admin_password_here"
    volumes:
      - ./data:/data
    restart: unless-stopped
```

Then run:
```bash
docker-compose up -d
```

<p align="right">(<a href="#docker-deployment">back to top</a>)</p>

## Contributing

Contributions are welcome! Please fork the repository, make your changes, and open a pull request.

1. Fork the Project on [Forgejo](https://git.prisma.moe/aichan/visit_counter)
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

Project Link: [https://git.prisma.moe/aichan/visit_counter](https://git.prisma.moe/aichan/visit_counter)

<p align="right">(<a href="#contact">back to top</a>)</p>

## Acknowledgments

This is based on [Anton Komarev's](https://komarev.com/anton) [github-profile-views-counter](https://github.com/antonkomarev/github-profile-views-counter), but made in Rust with a cool WebAssembly frontend.

Thanks to these amazing projects and technologies!

- [Rust Yew](https://yew.rs/) - A modern Rust framework for creating multi-threaded front-end web apps with WebAssembly
- [Rocket](https://rocket.rs/) - A web framework for Rust that makes it simple to write fast, secure web applications
- [WebAssembly](https://webassembly.org/) - A binary instruction format for a stack-based virtual machine
- [Font Awesome](https://fontawesome.com/) - Icons used in the web interface (Free License)

### Icon Attribution

This project uses Font Awesome icons, which are available under the Font Awesome Free License:

- **Icons**: CC BY 4.0 License (https://creativecommons.org/licenses/by/4.0/)
- **Fonts**: SIL OFL 1.1 License (https://scripts.sil.org/OFL)
- **Code**: MIT License (https://opensource.org/licenses/MIT)

The following Font Awesome icons are used in this project:
- `fa-chart-line`, `fa-shield-alt`, `fa-sync-alt`, `fa-list`, `fa-plus`, `fa-eye`, `fa-calendar`, `fa-clock`, `fa-edit`, `fa-trash`, `fa-code`, `fa-check`, `fa-copy`, `fa-key`, `fa-home`, `fa-cog`, `fa-sign-in-alt`, `fa-sign-out-alt`, `fa-sun`, `fa-moon`, `fa-magic`

<p align="right">(<a href="#readme-top">back to top</a>)</p>
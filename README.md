## Explanation of the Folder Structure:
Project Root:

* Cargo.toml & Cargo.lock: Define project dependencies and versions.
* README.md: Provide documentation and instructions for your project.
* .gitignore: Specify files and directories to be ignored by version control.
* .env: Keep environment variables (should be excluded from version control).
* 
src/:

* main.rs: The entry point of your application, where you set up your Axum app and run the server.
* lib.rs: An optional file where you can expose library functions if needed.

src/routes/:


* mod.rs: Module declarations and exports for routes.
* home.rs, users.rs: Route definitions for different sections/pages.

api/:


* mod.rs: Module declarations for API routes.
* auth.rs, items.rs: API route definitions.
  
src/handlers/:

* mod.rs: Module declarations for handlers.
* home_handler.rs, users_handler.rs: Implement the logic for handling requests to routes.
api/:
* mod.rs: Module declarations for API handlers.
* auth_handler.rs, items_handler.rs: Logic for API request handling.

src/models/:

* mod.rs: Module declarations for models.
* user.rs, item.rs: Define data structures and implement serialization/deserialization with Serde.

src/db/:
* mod.rs: Module declarations for database.
* connection.rs: Setup and manage the database connection.
* migrations/: SQL migration files for database schema.

src/services/:
* mod.rs: Module declarations for services.
* user_service.rs, item_service.rs: Business logic and interactions with the database.

src/utils/:
* mod.rs: Module declarations for utilities.
* config.rs: Configuration management, possibly using config crate.
* helpers.rs: General utility functions used across the application.

src/errors/:
* mod.rs: Module declarations for error handling.
* app_error.rs: Define custom error types and implementations.

src/middleware/:
* mod.rs: Module declarations for middleware.
* auth_middleware.rs: Middleware for authentication, logging, etc.

config/:
* development.yaml, production.yaml, test.yaml: Configuration files for different environments.

static/:
* css/, js/, images/: Static assets to be served by the application.
templates/:
* base.html, home.html: HTML templates for server-side rendering.

layout/:
* header.html: Shared layout components.
tests/:
* integration_tests.rs: Tests that check the integrity between different parts of the application.
* unit_tests.rs: Tests for individual units of code.
migrations/:
* Store your database migration scripts here if not within src/db/.
## Benefits of This Structure:
Separation of Concerns: Divides code logically by functionality, making it easier to find and maintain.
Scalability: Facilitates adding new features without cluttering existing code.
Team Collaboration: Allows multiple developers to work on different aspects simultaneously without merge conflicts.
Testing: Simplifies writing tests by clearly separating test code from application code.
Configuration Management: Stores environment-specific configurations separately, enhancing security and flexibility.



### Build Docker Image
```shell
sudo docker build -t axum_scaffold:debian -f Dockerfile.debian .

```

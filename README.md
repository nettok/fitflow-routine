# Routine

A fitness routine management API built with Rust, Axum, and Redis. This project allows users to manage workout routines, track assignments, and handle events for goal-based recommendations.

## Features

- **Routine Management**: Fetch routines categorized by training goals (e.g., Strength, Hypertrophy).
- **Assignment Tracking**: Track user assignments for routines, including status updates (Assigned, Started, Completed).
- **Event Handling**: Process events (e.g., goal setting) and recommend routines based on user goals.
- **Redis Integration**: Uses Redis for event queuing and real-time processing.

## Prerequisites

- Rust (latest stable version)
- Redis (for event handling and queuing)
- Docker (optional, for containerized deployment)

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/nettok/fitflow-routine.git
   cd fitflow-routine
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Configure the environment:
   - Set the `REDIS_URL` in the `.env` file, for example `REDIS_URL = "redis://localhost"`.

4. Start Redis:
   ```bash
   redis-server
   ```

## Running the Application

1. Start the server:
   ```bash
   cargo run
   ```

2. The server will be available at `http://localhost:8080`.

## API Endpoints

### Routines
- `GET /api/v1/routines/by-goal`: Fetch routines categorized by training goals.
- `GET /api/v1/routines/goals`: List all available training goals.

### Assignments
- `GET /api/v1/assignments/{user_id}`: Fetch all assignments for a user.
- `PUT /api/v1/assignments/{user_id}/accept/{routine_id}`: Accept a routine assignment.
- `PUT /api/v1/assignments/{user_id}/start/{routine_id}`: Mark a routine as started.
- `PUT /api/v1/assignments/{user_id}/complete/{routine_id}`: Mark a routine as completed.

## Deployment

### Fly.io
The project includes a `fly.toml` configuration for deployment on Fly.io. To deploy:
1. Install `flyctl` and log in.
2. Run:
   ```bash
   flyctl deploy
   ```

### Docker
The project includes a `Dockerfile` for containerized deployment. While primarily used by `flyctl` for deployment, you can also build and run the container locally for testing:

1. Build the Docker image:
   ```bash
   docker build -t routine .
   ```

2. Run the container (mapping port 8080):
   ```bash
   docker run -p 8080:8080 routine
   ```

The application will be available at `http://localhost:8080` as with the local development setup.

## License

This project is licensed under the AGPL License. See the `LICENSE` file for details.

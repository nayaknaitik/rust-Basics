use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{EnvFilter, fmt as subscriber_fmt};

// 1. The Todo model.
// Serialize: Converts Struct -> JSON (for sending responses).
// Deserialize: Converts JSON -> Struct (for reading requests).
#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

// Struct representing the request body when creating a new Todo.
#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

// Struct representing the request body when updating a Todo.
#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

// 2. Shared State.
// Actix Web is multi-threaded. We wrap our data in a Mutex so threads can access it safely.
struct AppState {
    todos: Mutex<Vec<Todo>>,
}

// 3. HANDLERS (endpoints)

// GET /todos - List all todos
#[get("/todos")]
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}
#[get("todos/getting")]
async fn greeting(data: web::Data<AppState>) -> impl Responder {
    let todo = data.todos.lock().unwrap();
    HttpResponse::Ok().json(&*todo)
}

// GET /todos/{id} - Get a single todo by ID
#[get("/todos/{id}")]
async fn get_todo_by_id(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    let id = path.into_inner();

    if let Some(todo) = todos.iter().find(|t| t.id == id) {
        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

// POST /todos - Create a new todo
#[post("/todos")]
async fn create_todo(new_todo: web::Json<CreateTodo>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();

    // Generate a simple ID
    let new_id = todos.last().map(|t| t.id + 1).unwrap_or(1);

    let todo = Todo {
        id: new_id,
        title: new_todo.title.clone(),
        completed: false,
    };

    todos.push(todo.clone());
    HttpResponse::Created().json(todo)
}

#[post("/check/health")]
async fn check_health() -> impl Responder {
    HttpResponse::Ok().body("Health is good and server is running fine")
}

// PUT /todos/{id} - Update an existing todo
#[put("/todos/{id}")]
async fn update_todo(
    path: web::Path<u32>,
    updated_fields: web::Json<UpdateTodo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = path.into_inner();

    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if let Some(ref title) = updated_fields.title {
            todo.title = title.clone();
        }
        if let Some(completed) = updated_fields.completed {
            todo.completed = completed;
        }
        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

// DELETE /todos/{id} - Delete a todo
#[delete("/todos/{id}")]
async fn delete_todo(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = path.into_inner();

    let initial_len = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() < initial_len {
        HttpResponse::Ok().body("Todo deleted successfully")
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

// 4. MAIN FUNCTION
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    subscriber_fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("🚀 Actix CRUD Server Starting on http://127.0.0.1:8080");

    // Initialize our shared in-memory database
    let app_state = web::Data::new(AppState {
        todos: Mutex::new(vec![
            Todo {
                id: 1,
                title: "Learn Rust".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Build an Actix Web app".to_string(),
                completed: false,
            },
        ]),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            // Register shared state so handlers can access it
            .app_data(app_state.clone())
            // Register services/endpoints
            .service(get_todos)
            .service(get_todo_by_id)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

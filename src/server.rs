use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::{todos::Todo, todos_server::Todos, TodoChangeResponse};

pub struct TodoService {
    todos: Arc<Mutex<HashMap<u32, Todo>>>,
}

impl Default for TodoService {
    fn default() -> Self {
        Self {
            todos: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl Todos for TodoService {
    #[doc = " Add to Todos"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn add(
        &self,
        request: Request<super::Todo>,
    ) -> Result<Response<super::TodoChangeResponse>, Status> {
        let todo = request.into_inner();
        let identifier = match todo.id.clone() {
            Some(id) => id,
            None => return Err(Status::invalid_argument("ID missing")),
        };

        let mut map = self.todos.lock().await;

        match map.get(&identifier.id) {
            Some(_) => return Err(Status::already_exists("ID already exists")),
            None => {
                map.insert(identifier.id, todo.clone());
                Ok(Response::new(TodoChangeResponse {
                    id: Some(identifier),
                    message: "New Todo added".to_string(),
                }))
            }
        }
    }

    #[doc = " Remove a todo"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn remove(
        &self,
        request: Request<super::TodoIdentifier>,
    ) -> Result<Response<super::TodoChangeResponse>, Status> {
        let identifier = request.into_inner();
        let mut map = self.todos.lock().await;
        match map.get(&identifier.id) {
            Some(_) => {
                map.remove(&identifier.id);
                Ok(Response::new(TodoChangeResponse {
                    id: Some(identifier),
                    message: "A ToDo removed".to_string(),
                }))
            }
            None => Err(Status::not_found("ID not found")),
        }
    }

    #[doc = " Update status of a Todo"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]

    async fn update_status(
        &self,
        request: Request<super::TodoStatusUpdateRequest>,
    ) -> Result<Response<super::TodoChangeResponse>, Status> {
        let request = request.into_inner();
        let mut map = self.todos.lock().await;

        let identifier = match request.id.clone() {
            Some(id) => id,
            None => return Err(Status::invalid_argument("ID missing")),
        };

        match map.get_mut(&identifier.id) {
            Some(todo) => {
                todo.status = request.status;
                Ok(Response::new(TodoChangeResponse {
                    id: Some(identifier),
                    message: "Status updated".to_string(),
                }))
            }
            None => Err(Status::not_found("ID not found")),
        }
    }

    #[doc = " Get a Todo by Identifier"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn get(
        &self,
        request: Request<super::TodoIdentifier>,
    ) -> Result<Response<super::Todo>, Status> {
        let identifier = request.into_inner();
        let map = self.todos.lock().await;
        match map.get(&identifier.id) {
            Some(todo) => Ok(Response::new(todo.clone())),
            None => Err(Status::not_found("ID not found")),
        }
    }
}

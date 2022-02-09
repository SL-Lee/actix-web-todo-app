export default class TodoApi {
  static async createTodo(todoTitle, todoContents) {
    return await (
      await fetch("/api/todos", {
        method: "POST",
        credentials: "same-origin",
        body: new URLSearchParams({
          todoTitle,
          todoContents: todoContents ? todoContents : "",
        }),
      })
    ).json();
  }

  static async getTodos() {
    return await (
      await fetch("/api/todos", {
        method: "GET",
        credentials: "same-origin",
      })
    ).json();
  }

  static async editTodo(todoId, todoTitle, todoContents) {
    return await (
      await fetch("/api/todos", {
        method: "PUT",
        credentials: "same-origin",
        body: new URLSearchParams({
          todoId,
          todoTitle,
          todoContents: todoContents ? todoContents : "",
        }),
      })
    ).json();
  }

  static async updateTodoStatus(todoId, newTodoStatus) {
    return await (
      await fetch("/api/todos", {
        method: "PATCH",
        credentials: "same-origin",
        body: new URLSearchParams({ todoId, todoCompleted: newTodoStatus }),
      })
    ).json();
  }

  static async deleteTodo(todoId) {
    return await (
      await fetch("/api/todos", {
        method: "DELETE",
        credentials: "same-origin",
        body: new URLSearchParams({ todoId }),
      })
    ).json();
  }
}

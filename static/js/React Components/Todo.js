import TodoDropdownMenu from "./TodoDropdownMenu.js";

class Todo extends React.Component {
  render() {
    return React.createElement(
      "div",
      { class: "col-12 col-md-6 col-xl-4" },
      React.createElement(
        "div",
        { class: "card m-15" },
        React.createElement("h2", { class: "card-title" }, this.props.title),
        React.createElement(
          "p",
          { class: "font-weight-bold" },
          "Status: ",
          React.createElement(
            "span",
            { class: `text-${this.props.completed ? "success" : "secondary"}` },
            this.props.completed ? "COMPLETED" : "INCOMPLETE"
          )
        ),
        React.createElement("hr", null, null),
        React.createElement(
          "p",
          {
            class: this.props.contents
              ? "text-truncate"
              : "text-muted font-italic",
          },
          this.props.contents ? this.props.contents : "Content not provided"
        ),
        React.createElement(
          TodoDropdownMenu,
          {
            id: this.props.id,
            title: this.props.title,
            contents: this.props.contents,
            completed: this.props.completed,
            updateTodoStatus: this.props.updateTodoStatus,
            editTodo: this.props.editTodo,
            deleteTodo: this.props.deleteTodo,
            showModal: this.props.showModal,
            hideModal: this.props.hideModal,
          },
          null
        )
      )
    );
  }
}

export default Todo;

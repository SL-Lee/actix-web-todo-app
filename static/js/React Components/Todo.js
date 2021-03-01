import DeleteTodoButton from "./DeleteTodoButton.js";
import EditTodoButton from "./EditTodoButton.js";
import UpdateTodoStatusButton from "./UpdateTodoStatusButton.js";

class Todo extends React.Component {
  handleClick = (e) => {
    this.props.showModal(
      {
        modalId: "view-todo-modal",
        modalTitle: this.props.title,
        showSecondaryButton: false,
        primaryButtonText: "Close",
        onOkHandler: (e) => halfmoon.toggleModal("view-todo-modal"),
      },
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
      this.props.contents
        ? React.createElement("p", null, this.props.contents)
        : React.createElement(
          "p",
          { class: "text-muted font-italic" },
          "Content not provided"
        )
    );
  }

  render() {
    return React.createElement(
      "div",
      { class: "col-12 col-md-6 col-xl-4" },
      React.createElement(
        "div",
        {
          class: "card m-15",
          style: { cursor: "pointer" },
          onClick: this.handleClick,
        },
        React.createElement(
          "h2",
          {
            class: "card-title",
            style: {
              textDecoration: this.props.completed ? "line-through" : "none",
            },
          },
          this.props.title
        ),
        this.props.contents
          ? React.createElement(
            "p",
            {
              class: "text-truncate",
              style: {
                textDecoration: this.props.completed
                  ? "line-through"
                  : "none",
              },
            },
            this.props.contents
          )
          : React.createElement(
            "p",
            { class: "text-muted font-italic" },
            "Content not provided"
          ),
        React.createElement(
          "div",
          { class: "btn-group", role: "group" },
          React.createElement(
            UpdateTodoStatusButton,
            {
              id: this.props.id,
              completed: this.props.completed,
              updateTodoStatus: this.props.updateTodoStatus,
            },
            null
          ),
          React.createElement(
            EditTodoButton,
            {
              id: this.props.id,
              title: this.props.title,
              contents: this.props.contents,
              showModal: this.props.showModal,
              hideModal: this.props.hideModal,
              editTodo: this.props.editTodo,
            },
            null
          ),
          React.createElement(
            DeleteTodoButton,
            {
              id: this.props.id,
              showModal: this.props.showModal,
              hideModal: this.props.hideModal,
              deleteTodo: this.props.deleteTodo,
            },
            null
          )
        )
      )
    );
  }
}

export default Todo;

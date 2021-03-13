import DeleteTodoButton from "./DeleteTodoButton.js";
import EditTodoButton from "./EditTodoButton.js";
import UpdateTodoStatusButton from "./UpdateTodoStatusButton.js";
import ViewTodoButton from "./ViewTodoButton.js";

class TodoDropdownMenu extends React.Component {
  render() {
    return React.createElement(
      "div",
      { class: "dropdown position-absolute" },
      React.createElement(
        "button",
        { class: "btn", "data-toggle": "dropdown" },
        React.createElement("i", { class: "fas fa-ellipsis-h" }, null)
      ),
      React.createElement(
        "div",
        { class: "dropdown-menu dropdown-menu-right" },
        React.createElement(
          ViewTodoButton,
          {
            title: this.props.title,
            contents: this.props.contents,
            completed: this.props.completed,
            showModal: this.props.showModal,
          },
          null
        ),
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
    );
  }
}

export default TodoDropdownMenu;

import EditTodoForm from "./EditTodoForm.js";

class EditTodoButton extends React.Component {
  handleClick = (e) => {
    e.stopPropagation();
    this.props.showModal(
      {
        modalId: "edit-todo-modal",
        modalTitle: "Edit to-do",
        primaryButtonText: "Edit",
        onOkHandler: (e) =>
          document.getElementById("edit-todo-form").requestSubmit(),
      },
      React.createElement(
        EditTodoForm,
        {
          modalId: "edit-todo-modal",
          formId: "edit-todo-form",
          todoId: this.props.id,
          todoTitle: this.props.title,
          todoContents: this.props.contents,
          hideModal: this.props.hideModal,
          editTodo: this.props.editTodo,
        },
        null
      )
    );
  };

  render() {
    return React.createElement(
      "a",
      {
        class: "dropdown-item",
        style: { cursor: "pointer" },
        onClick: this.handleClick,
      },
      React.createElement("i", { class: "mr-10 fas fa-edit" }, null),
      "Edit"
    );
  }
}

export default EditTodoButton;

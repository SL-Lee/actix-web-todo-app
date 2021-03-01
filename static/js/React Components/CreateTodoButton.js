import CreateTodoForm from "./CreateTodoForm.js";

class CreateTodoButton extends React.Component {
  handleClick = (e) => {
    this.props.showModal(
      {
        modalId: "create-todo-modal",
        modalTitle: "Create new to-do",
        primaryButtonText: "Create",
        onOkHandler: (e) =>
          document.getElementById("create-todo-form").requestSubmit(),
      },
      React.createElement(
        CreateTodoForm,
        {
          modalId: "create-todo-modal",
          formId: "create-todo-form",
          hideModal: this.props.hideModal,
          createTodo: this.props.createTodo,
        },
        null
      )
    );
  }

  render() {
    return React.createElement(
      "a",
      { class: "btn btn-primary", onClick: this.handleClick },
      React.createElement("i", { class: "fas fa-plus mr-5" }, null),
      "Create"
    );
  }
}

export default CreateTodoButton;

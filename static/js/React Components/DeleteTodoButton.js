class DeleteTodoButton extends React.Component {
  handleClick = (e) => {
    e.stopPropagation();
    this.props.showModal(
      {
        modalId: "delete-todo-modal",
        modalTitle: "Delete to-do?",
        primaryButtonText: "Delete",
        onOkHandler: (e) => {
          this.props.deleteTodo(this.props.id);
          this.props.hideModal("delete-todo-modal");
        },
      },
      React.createElement(
        "p",
        null,
        "Are you sure you want to delete this to-do?"
      )
    );
  }

  render() {
    return React.createElement(
      "a",
      { class: "btn btn-danger", onClick: this.handleClick },
      "Delete"
    );
  }
}

export default DeleteTodoButton;

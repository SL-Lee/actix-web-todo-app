class ViewTodoButton extends React.Component {
  handleClick = (e) => {
    e.stopPropagation();
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
  };

  render() {
    return React.createElement(
      "a",
      {
        class: "dropdown-item",
        style: { cursor: "pointer" },
        onClick: this.handleClick,
      },
      React.createElement("i", { class: "mr-10 fas fa-eye" }, null),
      "View"
    );
  }
}

export default ViewTodoButton;

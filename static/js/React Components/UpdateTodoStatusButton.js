class UpdateTodoStatusButton extends React.Component {
  handleClick = (e) => {
    e.stopPropagation();
    this.props.updateTodoStatus(this.props.id, !this.props.completed);
  };

  render() {
    return React.createElement(
      "a",
      {
        class: "dropdown-item",
        style: { cursor: "pointer" },
        onClick: this.handleClick,
      },
      React.createElement(
        "i",
        { class: `mr-10 fas fa-${this.props.completed ? "times" : "check"}` },
        null
      ),
      `Mark ${this.props.completed ? "Incomplete" : "Complete"}`
    );
  }
}

export default UpdateTodoStatusButton;

class UpdateTodoStatusButton extends React.Component {
  handleClick = (e) => {
    e.stopPropagation();
    this.props.updateTodoStatus(this.props.id, !this.props.completed);
  }

  render() {
    return React.createElement(
      "a",
      {
        class: `btn btn-${this.props.completed ? "secondary" : "success"}`,
        onClick: this.handleClick,
      },
      `Mark ${this.props.completed ? "Incomplete" : "Complete"}`
    );
  }
}

export default UpdateTodoStatusButton;

class TodoTitleInput extends React.Component {
  render() {
    return React.createElement(
      "div",
      { class: "form-group" },
      React.createElement("label", { for: "todo-title-input" }, "To-do Title"),
      React.createElement(
        "input",
        {
          id: "todo-title-input",
          class: "form-control",
          type: "text",
          name: "todoTitle",
          value: this.props.value,
          placeholder: "To-do Title",
          required: "required",
          onChange: this.props.handleTitleChange,
        },
        null
      )
    );
  }
}

export default TodoTitleInput;

class TodoContentsInput extends React.Component {
  render() {
    return React.createElement(
      "div",
      { class: "form-group" },
      React.createElement(
        "label",
        { for: "todo-title-input" },
        "To-do Contents"
      ),
      React.createElement(
        "textarea",
        {
          id: "todo-contents-input",
          class: "form-control",
          style: { resize: "none" },
          name: "todoContents",
          value: this.props.value,
          placeholder: "To-do Contents",
          onChange: this.props.handleContentsChange,
        },
        null
      )
    );
  }
}

export default TodoContentsInput;

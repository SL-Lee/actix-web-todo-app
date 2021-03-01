import TodoContentsInput from "./TodoContentsInput.js";
import TodoTitleInput from "./TodoTitleInput.js";

class CreateTodoForm extends React.Component {
  constructor(props) {
    super(props);
    this.state = { titleInput: "", contentsInput: "" };
  }

  handleSubmit = (e) => {
    e.preventDefault();
    this.props.createTodo(this.props.formId);
    this.props.hideModal(this.props.modalId);
    this.setState({ titleInput: "", contentsInput: "" });
  }

  handleTitleChange = (e) => {
    this.setState({ titleInput: e.target.value });
  }

  handleContentsChange = (e) => {
    this.setState({ contentsInput: e.target.value });
  }

  render() {
    return React.createElement(
      "form",
      { onSubmit: this.handleSubmit, id: this.props.formId },
      React.createElement(
        TodoTitleInput,
        {
          value: this.state.titleInput,
          handleTitleChange: this.handleTitleChange,
        },
        null
      ),
      React.createElement(
        TodoContentsInput,
        {
          value: this.state.contentsInput,
          handleContentsChange: this.handleContentsChange,
        },
        null
      ),
      React.createElement(
        "input",
        { type: "submit", style: { display: "none" } },
        null
      )
    );
  }
}

export default CreateTodoForm;

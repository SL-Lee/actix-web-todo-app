import TodoList from "./React Components/TodoList.js";

class App extends React.Component {
  render() {
    return [
      React.createElement("h1", null, "My To-dos"),
      React.createElement(TodoList, null, null),
    ];
  }
}

ReactDOM.render(
  React.createElement(App, null, null),
  document.getElementById("app-root")
);

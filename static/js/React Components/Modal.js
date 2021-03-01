class Modal extends React.Component {
  static defaultProps = {
    modalId: "modal",
    modalTitle: "Modal",
    children: null,
    showSecondaryButton: true,
    secondaryButtonText: "Cancel",
    primaryButtonText: "OK",
    onOkHandler: () => { },
  };

  render() {
    return ReactDOM.createPortal(
      React.createElement(
        "div",
        { class: "modal", id: this.props.modalId, role: "dialog" },
        React.createElement(
          "div",
          { class: "modal-dialog", role: "document" },
          React.createElement(
            "div",
            { class: "modal-content" },
            React.createElement(
              "a",
              { class: "close", role: "button", "data-dismiss": "modal" },
              "×"
            ),
            React.createElement(
              "h5",
              { class: "modal-title" },
              this.props.modalTitle
            ),
            this.props.children,
            React.createElement(
              "div",
              { class: "text-right mt-20" },
              this.props.showSecondaryButton
                ? React.createElement(
                  "a",
                  {
                    class: "btn mr-5",
                    role: "button",
                    "data-dismiss": "modal",
                  },
                  this.props.secondaryButtonText
                )
                : null,
              React.createElement(
                "a",
                {
                  class: "btn btn-primary",
                  role: "button",
                  onClick: this.props.onOkHandler,
                },
                this.props.primaryButtonText
              )
            )
          )
        )
      ),
      document.getElementById("modal-root")
    );
  }
}

export default Modal;

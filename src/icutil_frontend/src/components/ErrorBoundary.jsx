import { Component } from 'react';

export class ErrorBoundary extends Component {
  state = { error: null };

  static getDerivedStateFromError(error) {
    return { error };
  }

  render() {
    if (this.state.error) {
      return (
        <div className="error-fallback">
          <h2>Something went wrong</h2>
          <details>{this.state.error.toString()}</details>
        </div>
      );
    }
    return this.props.children;
  }
}
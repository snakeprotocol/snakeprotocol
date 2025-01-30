import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter as Router } from 'react-router-dom';
import App from './App';

// Error Boundary Component
class ErrorBoundary extends React.Component<{ children: React.ReactNode }, { hasError: boolean }> {
  constructor(props: { children: React.ReactNode }) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(_: Error) {
    return { hasError: true };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    // Send error to main process
    window.electron.logInfo(`[ERROR] ${error.toString()}\n${errorInfo.componentStack}`);
  }

  render() {
    if (this.state.hasError) {
      return <h1>Something went wrong.</h1>;
    }

    return this.props.children;
  }
}

// Set up console interceptors
const originalConsole = {
  log: console.log,
  error: console.error,
  warn: console.warn,
  info: console.info,
};

// Intercept console methods
console.log = (...args) => {
  window.electron.logInfo(`[LOG] ${args.join(' ')}`);
  originalConsole.log(...args);
};

console.error = (...args) => {
  window.electron.logInfo(`[ERROR] ${args.join(' ')}`);
  originalConsole.error(...args);
};

console.warn = (...args) => {
  window.electron.logInfo(`[WARN] ${args.join(' ')}`);
  originalConsole.warn(...args);
};

console.info = (...args) => {
  window.electron.logInfo(`[INFO] ${args.join(' ')}`);
  originalConsole.info(...args);
};

// Capture unhandled promise rejections
window.addEventListener('unhandledrejection', (event) => {
  window.electron.logInfo(`[UNHANDLED REJECTION] ${event.reason}`);
});

// Capture global errors
window.addEventListener('error', (event) => {
  window.electron.logInfo(
    `[GLOBAL ERROR] ${event.message} at ${event.filename}:${event.lineno}:${event.colno}`
  );
});

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ErrorBoundary>
      <Router>
        <App />
      </Router>
    </ErrorBoundary>
  </React.StrictMode>
);

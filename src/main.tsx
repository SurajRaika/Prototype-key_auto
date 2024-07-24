import React, { useRef } from "react";
import ReactDOM from "react-dom/client";
import {  useEffect } from "react";
import { BrowserRouter } from 'react-router-dom';

import { Provider } from "react-redux";
import { store } from "./store/EventStore";
import {  useAppDispatch } from "./hooks";

import App from "./App";
import ListenGlobalEvent from "./EventListener"; // Import the global event listener

/**
 * The main component of the application.
 * This component initializes the global event listener and renders the main application component.
 *
 * @returns {React.ReactElement} - The rendered component.
 */
const Main = () => {
  const dispatch = useAppDispatch(); // The dispatch function from the Redux store
  const hasLoaded = useRef(false); // A reference to track if the component has been loaded

  /**
   * An effect hook that initializes the global event listener when the component mounts.
   * The listener is only initialized once, by checking the value of `hasLoaded.current`.
   */
  useEffect(() => {
    if (!hasLoaded.current) {
      ListenGlobalEvent(dispatch); // Initialize the global event listener
      hasLoaded.current = true; // Update the loaded state
    }
  }, []);

  return <App />; // Render the main application component
};
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Provider store={store}>
    <BrowserRouter>
      <Main />
</BrowserRouter>
    </Provider>
  </React.StrictMode>
);

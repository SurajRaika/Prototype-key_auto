import React from 'react';
import { Routes, Route,Link } from 'react-router-dom';
import Home from './components/pages/Home';
import Navbar from './components/NavigationBar';

import CreateAction from './components/pages/CreateAction';
// import NotFound from './components/pages/NotFound';
import './App.css';

function App() {
  return (
    <div className="App">
      <Navbar/>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/NewAction" element={<CreateAction />} />
        {/* <Route path="*" element={<NotFound />} /> */}
      </Routes>
    </div>
  );
}

export default App;

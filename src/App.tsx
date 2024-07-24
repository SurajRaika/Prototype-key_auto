import React from 'react';
import { Routes, Route,Link } from 'react-router-dom';
import Home from './components/pages/home.tsx';
import NewAction from './components/pages/new.tsx';
// import NotFound from './components/pages/NotFound';
import './App.css';

function App() {
  return (
    <div className="App">
                <Link to="/">Home</Link>  
                <br />
                <Link to="/NewAction">NewAction</Link>

      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/NewAction" element={<NewAction />} />
        {/* <Route path="*" element={<NotFound />} /> */}
      </Routes>
    </div>
  );
}

export default App;

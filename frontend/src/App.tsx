import { Route, Routes } from "react-router-dom";
import Home from "./Home";
import Game from "./Game";

function App() {
  return (
    <>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/:game_id" element={<Game />} />
      </Routes>
    </>
  );
}

export default App;

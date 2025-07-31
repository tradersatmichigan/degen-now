import { useParams } from "react-router-dom"

function Game() {
  const { game_id } = useParams();

  return (
    <>
      Game: {game_id}
    </>
  )
}

export default Game

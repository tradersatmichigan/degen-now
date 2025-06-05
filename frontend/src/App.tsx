import { Box, Paper } from "@mui/material";
import { useState } from "react";

interface Player {
  stack: number
  name: string
}

interface GameState {
  players: Player[]
}

export function App() {
  
  const [gameState, setState] = useState<GameState>({
    players: [
      {
        stack: 100,
        name: "Bob",
      }
    ]
  })

  return (
    <Box 
      className="poker-table-container"
      sx={{
        width: '100vw',
        height: '100vh',
        backgroundColor: '#212121',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
      }}
    >
      
      <Paper 
        className="poker-table" 
        elevation={3}
        sx={{
          width: '65vw',
          height: '50vh',
          backgroundColor: 'green',
          borderRadius: '50%',
        }}
      >
      </Paper>

    </Box>
  );
}

export default App;

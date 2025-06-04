import { Box, Paper } from "@mui/material";

export function App() {
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

import React from 'react';

const PokerTable: React.FC = () => {
  return (
    <div className="flex items-center justify-center h-screen bg-gray-900">
      <div 
      className="w-[500px] h-[300px] 
      bg-green-700 rounded-full shadow-2xl border-4 border-green-900">
        {/* You can add cards, chips, or players around here */}
      </div>
    </div>
  );
};

function App() {
  return (
    <div className="App">
      <PokerTable/>
    </div>
  );
}

export default App;

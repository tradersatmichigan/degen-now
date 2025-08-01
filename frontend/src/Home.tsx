import React from "react";

const Home: React.FC = () => {
  const handleCreateGame = () => {
    console.log("Create game clicked!");
    // Add your game creation logic here
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 flex items-center justify-center p-4">
      <div className="text-center space-y-8">
        {/* App Name */}
        <div className="space-y-2">
          <h1 className="text-6xl md:text-8xl font-bold bg-gradient-to-r from-cyan-400 via-purple-400 to-pink-400 bg-clip-text text-transparent animate-pulse">
            degen now
          </h1>
          <div className="h-1 w-32 bg-gradient-to-r from-cyan-400 to-pink-400 mx-auto rounded-full"></div>
        </div>

        {/* Create Game Button */}
        <button
          onClick={handleCreateGame}
          className="group relative px-8 py-4 bg-gradient-to-r from-purple-600 to-pink-600 text-white font-semibold text-xl rounded-xl shadow-2xl hover:shadow-purple-500/25 transform hover:scale-105 transition-all duration-300 ease-out"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-purple-600 to-pink-600 rounded-xl blur opacity-75 group-hover:opacity-100 transition-opacity duration-300"></div>
          <span className="relative flex items-center gap-2">
            Create Game
            <svg
              className="w-6 h-6 group-hover:translate-x-1 transition-transform duration-300"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 7l5 5m0 0l-5 5m5-5H6"
              />
            </svg>
          </span>
        </button>

        {/* Decorative elements */}
        <div className="absolute top-20 left-20 w-32 h-32 bg-purple-500/10 rounded-full blur-xl animate-bounce"></div>
        <div className="absolute bottom-20 right-20 w-24 h-24 bg-pink-500/10 rounded-full blur-xl animate-bounce delay-1000"></div>
      </div>
    </div>
  );
};

export default Home;

import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { spawn } from 'child_process';

function runBackend() {
  let backend: any;

  return {
    name: 'rust-backend',
    configureServer() {
      if (!backend) {
        backend = spawn('cargo', ['run'], {
          cwd: '../backend',
          stdio: 'inherit',
          shell: true,
        });
      }
    },
    closeBundle() {
      if (backend) backend.kill();
    }
  };
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss(), runBackend()],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
})

import type { CreateGameResponse } from "./types";

export const createGame = async () => {
  return callApi<CreateGameResponse>("/create");
}

// endpoint should be path without /api prefix
const callApi = async <T>(endpoint: string, data: any = {}) => {
  const response = await fetch("/api" + endpoint, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });

  if (response.ok) {
    return response.json() as T;
  } else {
    console.error(response.body);
    return null;
  }
}

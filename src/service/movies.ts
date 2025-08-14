import axios from 'axios';

export async function GetMovies() {
    const response = await axios.get(`${window.API_URL}/movies`);
    return response.data;
}

export default {
    GetMovies
};
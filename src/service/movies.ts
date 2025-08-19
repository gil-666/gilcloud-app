import axios from 'axios';
import { useApiUrl } from '../main';

export async function GetMovies() {
    const response = await axios.get(`${useApiUrl()}/movies`);
    return response.data;
}

export default {
    GetMovies
};
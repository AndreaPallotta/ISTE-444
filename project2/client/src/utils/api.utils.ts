import axios from 'axios';

interface ApiResponseError {
    error_msg: string;
}

interface ApiResponse<T> {
    result: string;
    content: ApiResponseError | T;
}

interface AxiosResponse<T> {
    status?: number;
    data?: ApiResponse<T>;
    error?: string;
}

const prefix = import.meta.env.DEV ? 'http://localhost:3000' : 'ransapi.iste444.com';

export const client = axios.create({
    baseURL: `http://${prefix}`,
    headers: {
        'Content-Type': 'application/json',
    },
});

export async function axiosGet<T, B>(endpoint: string, params: B | undefined): Promise<AxiosResponse<T>> {
    try {
        const { status, data } = await client.get(endpoint, { data: params });
        return { status, data };
    } catch (err) {
        if (axios.isAxiosError(err)) {
            const { data, status } = err.response ?? {};
            return { error: data.error, status };
        }
        return { error: 'Request failed with unknown error' };
    }
};

export async function axiosPost<T, B>(endpoint: string, body: B): Promise<AxiosResponse<T>> {
    try {
        const { status, data } = await client.post(endpoint, body);
        return { status, data };
    } catch (err) {
        if (axios.isAxiosError(err)) {
            const { data, status } = err.response ?? {};
            return { error: data.error, status };
        }
        return { error: 'Request failed with unknown error' };
    }
};

export async function axiosPut<T, B>(endpoint: string, body: B): Promise<AxiosResponse<T>> {
    try {
        const { status, data } = await client.put(endpoint, body);
        return { status, data };
    } catch (err) {
        if (axios.isAxiosError(err)) {
            const { data, status } = err.response ?? {};
            return { error: data.error, status };
        }
        return { error: 'Request failed with unknown error' };
    }
};

export async function axiosDelete<T, B>(endpoint: string, body: B): Promise<AxiosResponse<T>> {
    try {
        const { status, data } = await client.delete(endpoint, { data: body });
        return { status, data };
    } catch (err) {
        if (axios.isAxiosError(err)) {
            const { data, status } = err.response ?? {};
            return { error: data.error, status };
        }
        return { error: 'Request failed with unknown error' };
    }
};
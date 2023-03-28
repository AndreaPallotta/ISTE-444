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

const prefix = 'api.project2.com';

export const client = axios.create({
    baseURL: `http://${prefix}`,
    headers: {
        'Content-Type': 'application/json',
    },
});

type RequestType = 'get' | 'post' | 'put' | 'delete';

export async function get<T>(endpoint: string, params: T): Promise<AxiosResponse<T>> {
    try {
        const { status, data } = await get(endpoint, params);
        return { status, data };
    } catch (err) {
        if (axios.isAxiosError(err)) {
            const { data, status } = err.response ?? {};
            return { error: data.error, status };
        }
        return { error: 'Request failed with unknown error' };
    }
};

export async function post<T>(endpoint: string, body: T): Promise<AxiosResponse<T>> {
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

export async function put<T>(endpoint: string, body: T): Promise<AxiosResponse<T>> {
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

export async function deleteFn<T>(endpoint: string, body: T): Promise<AxiosResponse<T>> {
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
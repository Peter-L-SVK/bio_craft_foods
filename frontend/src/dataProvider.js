import { fetchUtils } from 'react-admin';
import { stringify } from 'query-string';

const apiUrl = 'http://localhost:3000/api';
const httpClient = fetchUtils.fetchJson;

const dataProvider = {
    getList: async (resource, params) => {
    const { page, perPage } = params.pagination;
    const { field, order } = params.sort;
    const query = {
        sort: JSON.stringify([field, order]),
        range: JSON.stringify([(page - 1) * perPage, page * perPage - 1]),
        filter: JSON.stringify(params.filter),
    };
    const url = `${apiUrl}/${resource}?${stringify(query)}`;

    console.log("Fetching data from:", url); // Log the URL

    try {
        const { json } = await httpClient(url);
        console.log("Backend response:", json); // Log the backend response

        const transformedData = {
            data: json.data.map(resource => ({ ...resource, id: resource.id })),
            total: json.total,
        };

        console.log("Transformed data:", transformedData); // Log the transformed data

        return transformedData;
    } catch (error) {
        console.error("Error fetching data:", error); // Log any errors
        throw error;
     }
    },
    getOne: async (resource, params) => {
        const url = `${apiUrl}/${resource}/${params.id}`;
        const { json } = await httpClient(url);
        return {
            data: json.data,
        };
    },
    getMany: async (resource, params) => {
        const query = {
            filter: JSON.stringify({ id: params.ids }),
        };
        const url = `${apiUrl}/${resource}?${stringify(query)}`;
        const { json } = await httpClient(url);
        return {
            data: json.data,
        };
    },
    getManyReference: async (resource, params) => {
        const { page, perPage } = params.pagination;
        const { field, order } = params.sort;
        const query = {
            sort: JSON.stringify([field, order]),
            range: JSON.stringify([(page - 1) * perPage, page * perPage - 1]),
            filter: JSON.stringify({
                ...params.filter,
                [params.target]: params.id,
            }),
        };
        const url = `${apiUrl}/${resource}?${stringify(query)}`;
        const { json } = await httpClient(url);
        return {
            data: json.data,
            total: json.total,
        };
    },
    update: async (resource, params) => {
        const url = `${apiUrl}/${resource}/${params.id}`;
        const { json } = await httpClient(url, {
            method: 'PUT',
            body: JSON.stringify(params.data),
        });
        return {
            data: json.data,
        };
    },
    updateMany: async (resource, params) => {
        const url = `${apiUrl}/${resource}`;
        const { json } = await httpClient(url, {
            method: 'PUT',
            body: JSON.stringify(params.ids),
        });
        return {
            data: json.data,
        };
    },
    create: async (resource, params) => {
        const url = `${apiUrl}/${resource}`;
        console.log("Creating resource at:", url); // Log the URL
        console.log("Request payload:", params.data); // Log the payload

        try {
            const { json } = await httpClient(url, {
                method: 'POST',
                body: JSON.stringify(params.data),
            });
            console.log("Backend response:", json); // Log the backend response
            return {
                data: { ...params.data, id: json.id },
            };
        } catch (error) {
            console.error("Error creating resource:", error); // Log any errors
            throw error;
        }
    },
    delete: async (resource, params) => {
        const url = `${apiUrl}/${resource}/${params.id}`;
        const { json } = await httpClient(url, {
            method: 'DELETE',
        });
        return {
            data: json.data,
        };
    },
    deleteMany: async (resource, params) => {
        //const url = `${apiUrl}/${resource}`;
        const url = `${apiUrl}/${resource}/bulk-delete`;

        console.log("Deleting multiple records from:", url); // Log the URL
        console.log("IDs to delete:", params.ids); // Log the IDs to delete

        try {
            const { json } = await httpClient(url, {
                //method: 'DELETE',
                method: 'POST',
                body: JSON.stringify(params.ids),
            });
            console.log("Backend response for deleteMany:", json); // Log the backend response
            return {
                data: json.data || params.ids, // Ensure the response includes the deleted IDs
            };
        } catch (error) {
            console.error("Error deleting multiple records:", error); // Log any errors
            throw error;
        }
    },
};

export default dataProvider;
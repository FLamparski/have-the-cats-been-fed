import { useMutation, useQuery, useQueryClient } from "@tanstack/vue-query";
import { get, set } from "idb-keyval";

const queryKey = ['deviceId'];

export function useDeviceId() {
    return useQuery({
        queryKey,
        queryFn: () => get('deviceId'),
    });
}

export function useSetDeviceIdMutation() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: (deviceId: string) => set('deviceId', deviceId),
        onSettled: () => queryClient.invalidateQueries({ queryKey })
    });
}

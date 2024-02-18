import { useQuery, useMutation } from "@tanstack/vue-query";

const queryKey = ['log'];

export function useFeedingLogQuery() {
    return useQuery({
        queryKey,
        queryFn: () => $fetch('/backend/log?size=1'),
    })
}

export function useFeedMutation () {
    const queryClient = useQueryClient();
  const { data: deviceId } = useDeviceId();
  return useMutation({
    mutationFn: () => $fetch('/backend/log', { method: 'POST', body: { device_id: deviceId.value } }),
    onSettled: () => queryClient.invalidateQueries({ queryKey }),
  });
};

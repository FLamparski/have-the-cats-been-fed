import { useMutation } from "@tanstack/vue-query";
import axios from "axios";

export const useFeedMutation = function () {
  const { data: deviceId } = useDeviceId();
  return useMutation({
    mutationFn: () => axios.post("/backend/log", { device_id: deviceId.value }),
  });
};

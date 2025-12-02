import { useQuery } from '@tanstack/react-query';
import { useActor } from './useActor';

export function useStepTemplates() {
  const { actor, isFetching } = useActor();

  return useQuery<Array<[string, string]>>({
    queryKey: ['stepTemplates'],
    queryFn: async () => {
      if (!actor) return [];
      return actor.getAllStepTemplates();
    },
    enabled: !!actor && !isFetching,
  });
}

export function useTooltips() {
  const { actor, isFetching } = useActor();

  return useQuery<Array<[string, string]>>({
    queryKey: ['tooltips'],
    queryFn: async () => {
      if (!actor) return [];
      return actor.getAllTooltips();
    },
    enabled: !!actor && !isFetching,
  });
}

export function useHelpTexts() {
  const { actor, isFetching } = useActor();

  return useQuery<Array<[string, string]>>({
    queryKey: ['helpTexts'],
    queryFn: async () => {
      if (!actor) return [];
      return actor.getAllHelpTexts();
    },
    enabled: !!actor && !isFetching,
  });
}

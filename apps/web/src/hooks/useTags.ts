import { useQuery } from '@tanstack/react-query'
import { create, windowScheduler } from '@yornaath/batshit'
import { postTagsBatch } from '../client/sdk.gen'
import type { Tag, TagId } from '../client/types.gen'

const tagsBatcher = create({
  fetcher: async (ids: TagId[]) => {
    const { data } = await postTagsBatch({
      body: ids,
    })
    return data || []
  },
  resolver: (items: Tag[], query: TagId) =>
    items.find((item) => item.id === query) ?? null,
  scheduler: windowScheduler(10),
})

export function useTag(tagId: TagId | undefined) {
  return useQuery({
    queryKey: ['tag', tagId],
    queryFn: async () => {
      if (!tagId) return null
      return tagsBatcher.fetch(tagId)
    },
    enabled: !!tagId,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

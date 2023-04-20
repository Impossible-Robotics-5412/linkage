<script lang="ts">

</script>

<div class='list'>
	<slot/>
</div>

<style lang="scss">
	@use '../../style/vars' as *;

    .list {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      & :global(> *) {
        border-bottom: 1px solid $c-gray-1;
        padding-bottom: 0.5rem;

        &:last-child {
          border-bottom: none;
          padding-bottom: 0;
        }
      }

      & :global(h3:not(:first-child)) {
        padding-top: 1.5rem;
      }
    }
</style>
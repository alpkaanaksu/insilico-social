<script>
    import { Academia } from "insilico-social-core";

    const academia = new Academia();

    academia.init(5);

    let summary = $state(academia.summarize());



    academia.run(12000000, 100, 12, (s) => {
       summary = s;
    })
</script>

<div class="model-container">
    <h3>model</h3>
    <table>
        <tbody>
            {#each Object.entries(summary) as [key, value]}
                <tr>
                    <td>{key}</td>
                    <td>{value}</td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>


<style>
    .model-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 100%;
    }
</style>


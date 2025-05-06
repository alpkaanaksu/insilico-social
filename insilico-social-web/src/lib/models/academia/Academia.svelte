<script>
    import { Academia } from "insilico-social-core";

    const academia = new Academia();

    const r = academia.serialize();

    console.log(r);
</script>

<div class="model-container">
    <h3>model</h3>
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


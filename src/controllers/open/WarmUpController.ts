import {Controller, Get, HttpStatus} from "@nestjs/common";

@Controller('_ah')
export class WarmUpController {

    @Get('warmup')
    warmUp() {
        return HttpStatus.OK;
    }

}

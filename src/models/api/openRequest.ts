/**
 * Sesamo
 * APIs for Sesamo project
 *
 * OpenAPI spec version: 0.0.1
 * Contact: matteopietro.dazzi@gmail.com
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 * Do not edit the class manually.
 */

export interface OpenRequest {
    /**
     * ID of the device that want to open the object
     */
    deviceId: string;
    /**
     * ID of the server that will handle the request
     */
    serverId: string;
}